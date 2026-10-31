use crate::Error;
use std::sync::{Arc, Mutex};
use tokio::sync::{Notify, Semaphore};

/// Adapter-private executor coordination, never part of semantic request/result values.
pub(crate) struct Jobs {
    active: Arc<Semaphore>,
    limit: usize,
    state: Mutex<State>,
    changed: Notify,
}
struct State {
    admitted: usize,
    closed: bool,
}
struct Admission(Arc<Jobs>);
impl Drop for Admission {
    fn drop(&mut self) {
        self.0.state.lock().unwrap().admitted -= 1;
        self.0.changed.notify_waiters();
    }
}

impl Jobs {
    pub fn new(active: usize, waiting: usize) -> Result<Arc<Self>, Error> {
        if active == 0 || active > 65_536 || waiting > 65_536 {
            return Err(Error::InvalidInput(
                "finite job limits: 1..=65536 active, 0..=65536 waiting".into(),
            ));
        }
        Ok(Arc::new(Self {
            active: Arc::new(Semaphore::new(active)),
            limit: active + waiting,
            state: Mutex::new(State {
                admitted: 0,
                closed: false,
            }),
            changed: Notify::new(),
        }))
    }

    pub async fn run<T, F>(self: &Arc<Self>, job: F) -> Result<T, Error>
    where
        T: Send + 'static,
        F: FnOnce() -> Result<T, Error> + Send + 'static,
    {
        let runtime = tokio::runtime::Handle::try_current()
            .map_err(|_| Error::Unavailable("host-owned Tokio runtime required".into()))?;
        {
            let mut state = self.state.lock().unwrap();
            if state.closed {
                return Err(Error::Unavailable(
                    "backend drained; admission closed".into(),
                ));
            }
            if state.admitted == self.limit {
                return Err(Error::Capacity);
            }
            state.admitted += 1;
        }
        let admission = Admission(self.clone());
        let active = self.active.clone();
        // The detached coordinator owns admission through queueing AND completion.
        // Dropping a caller's JoinHandle does not abort the coordinator or its blocking job.
        let task = runtime.spawn(async move {
            let permit = active
                .acquire_owned()
                .await
                .map_err(|_| Error::Unavailable("job admission closed".into()))?;
            tokio::task::spawn_blocking(move || {
                let (_admission, _permit) = (admission, permit);
                job()
            })
            .await
            .map_err(|_| Error::Uncertain {
                operation_id: None,
                stage: "blocking job interrupted or panicked; resolve original operation".into(),
            })?
        });
        task.await.map_err(|_| Error::Uncertain {
            operation_id: None,
            stage: "job coordinator stopped; resolve original operation".into(),
        })?
    }

    pub async fn drain(&self) {
        loop {
            let notified = self.changed.notified();
            {
                let mut state = self.state.lock().unwrap();
                state.closed = true;
                if state.admitted == 0 {
                    return;
                }
            }
            notified.await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(flavor = "current_thread")]
    async fn cancelled_wait_keeps_capacity_and_drain_waits() {
        let jobs = Jobs::new(1, 1).unwrap();
        let (started, rx) = std::sync::mpsc::channel();
        let (release, wait) = std::sync::mpsc::channel();
        let j = jobs.clone();
        let task = tokio::spawn(async move {
            j.run(move || {
                started.send(()).unwrap();
                wait.recv().unwrap();
                Ok(())
            })
            .await
        });
        while rx.try_recv().is_err() {
            tokio::task::yield_now().await;
        }
        task.abort();
        let j = jobs.clone();
        let queued = tokio::spawn(async move { j.run(|| Ok(7)).await });
        tokio::task::yield_now().await;
        let mutated = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mutation = mutated.clone();
        assert_eq!(
            jobs.run(move || {
                mutation.store(true, std::sync::atomic::Ordering::SeqCst);
                Ok(1)
            })
            .await,
            Err(Error::Capacity)
        );
        assert!(!mutated.load(std::sync::atomic::Ordering::SeqCst));
        let j = jobs.clone();
        let drain = tokio::spawn(async move { j.drain().await });
        tokio::task::yield_now().await;
        assert!(!drain.is_finished());
        release.send(()).unwrap();
        assert_eq!(queued.await.unwrap().unwrap(), 7);
        drain.await.unwrap();
        assert!(matches!(
            jobs.run(|| Ok(1)).await,
            Err(Error::Unavailable(_))
        ));
    }
}
