#![allow(dead_code)]
use knowledgehub::{backends::files::*, *};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
pub struct Sandbox {
    pub parent: PathBuf,
    pub store: PathBuf,
}
impl Sandbox {
    pub fn new() -> Self {
        let parent = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/kh-tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        std::fs::create_dir_all(&parent).unwrap();
        let store = parent.join("store");
        Self { parent, store }
    }
    pub fn backend(&self) -> FileBackend {
        FileBackend::new(&self.store, FileOptions::native_local().unwrap()).unwrap()
    }
    pub async fn create(&self) -> KnowledgeHub {
        KnowledgeHub::create(self.backend(), crate::seed::context(), crate::seed::scope())
            .await
            .unwrap()
    }
    pub async fn open(&self) -> Result<KnowledgeHub, Error> {
        KnowledgeHub::open(self.backend(), crate::seed::context(), crate::seed::scope()).await
    }
}
impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.parent);
    }
}
