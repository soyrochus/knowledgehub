//! Native mechanisms only; accepted model and file-format semantics stay in the adapter.
use super::*;

pub(super) const EXT4: &str = "linux-x86_64/local-ext4/single-host/1";
pub(super) const APFS: &str = "macos-aarch64/local-apfs/single-host/1";

pub(super) fn same_device(expected: u64, observed: u64) -> Result<(), Error> {
    if expected != observed {
        return Err(Error::Incompatible(
            "files/4 directories cross filesystems".into(),
        ));
    }
    Ok(())
}

pub(super) fn declared(support: &FileSupport) -> Result<&'static str, Error> {
    match (
        support.host.as_str(),
        support.filesystem.as_str(),
        support.coordinated_single_host,
    ) {
        ("linux-x86_64", "ext4", true) => Ok(EXT4),
        ("macos-aarch64", "apfs", true) => Ok(APFS),
        _ => Err(Error::Incompatible(
            "coordinated single-host local ext4/APFS declaration required".into(),
        )),
    }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn apfs_observation(filesystem: &[u8], flags: u32) -> Result<(), Error> {
    if filesystem != b"apfs"
        || flags & libc::MNT_LOCAL as u32 == 0
        || flags & libc::MNT_RDONLY as u32 != 0
    {
        return Err(Error::Incompatible(
            "detected filesystem is outside writable local APFS profile".into(),
        ));
    }
    Ok(())
}

pub(super) fn check(support: &FileSupport, path: &Path) -> Result<(), Error> {
    let declared = declared(support)?;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let expected = EXT4;
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let expected = APFS;
    #[cfg(not(any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64")
    )))]
    let expected = "unsupported native host";
    if declared != expected {
        return Err(Error::Incompatible(
            "declared file profile differs from native host".into(),
        ));
    }
    is_dir(path)?;
    let file = File::open(path).map_err(|_| storage("open filesystem profile"))?;
    let mut stat = std::mem::MaybeUninit::<libc::statfs>::uninit();
    // SAFETY: stat has sufficient writable storage; file owns a live descriptor.
    if unsafe { libc::fstatfs(file.as_raw_fd(), stat.as_mut_ptr()) } != 0 {
        return Err(storage("inspect filesystem profile"));
    }
    // SAFETY: successful fstatfs initialized stat.
    let stat = unsafe { stat.assume_init() };
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        if stat.f_type != 0xef53 {
            return Err(Error::Incompatible(
                "detected filesystem is outside local ext4 profile".into(),
            ));
        }
        // ext2/ext3 share this magic: retain actual mounted ext4 confirmation.
        let dev = file
            .metadata()
            .map_err(|_| storage("inspect mount device"))?
            .dev();
        let device = format!("{}:{}", libc::major(dev), libc::minor(dev));
        let mounts = fs::read_to_string("/proc/self/mountinfo")
            .map_err(|_| Error::Unavailable("cannot establish mounted filesystem type".into()))?;
        if !mounts
            .lines()
            .filter_map(|line| line.split_once(" - "))
            .any(|(left, right)| {
                left.split_whitespace().nth(2) == Some(device.as_str())
                    && right.split_whitespace().next() == Some("ext4")
            })
        {
            return Err(Error::Incompatible(
                "mounted device is outside ext4 profile".into(),
            ));
        }
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        // SAFETY: Darwin initializes the bounded, NUL-terminated filesystem name.
        let name = unsafe { std::ffi::CStr::from_ptr(stat.f_fstypename.as_ptr()) };
        apfs_observation(name.to_bytes(), stat.f_flags)?;
    }
    #[cfg(not(any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64")
    )))]
    let _ = stat;
    Ok(())
}

pub(super) fn sync_file(file: &File) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        // SAFETY: live regular-file descriptor; no weaker fallback on failure.
        if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_FULLFSYNC) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    file.sync_all()
}

pub(super) fn sync_directory(path: &Path) -> std::io::Result<()> {
    let file = File::open(path)?;
    #[cfg(target_os = "macos")]
    {
        // SAFETY: live directory descriptor; directory fsync is a distinct Darwin primitive.
        if unsafe { libc::fsync(file.as_raw_fd()) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    file.sync_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn declarations_are_explicit_and_coordination_is_required() {
        assert!(same_device(1, 1).is_ok());
        assert!(same_device(1, 2).is_err()); // Injected device observation, no mounted test volume.
        assert_eq!(declared(&FileSupport::local_ext4()).unwrap(), EXT4);
        assert_eq!(declared(&FileSupport::local_apfs()).unwrap(), APFS);
        let mut support = FileSupport::local_apfs();
        support.coordinated_single_host = false;
        assert!(declared(&support).is_err());
        support.coordinated_single_host = true;
        support.filesystem = "ext4".into();
        assert!(declared(&support).is_err());
    }
    #[test]
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    fn injected_apfs_observations_refuse_nonlocal_readonly_and_other_filesystems() {
        assert!(apfs_observation(b"apfs", libc::MNT_LOCAL as u32).is_ok());
        assert!(apfs_observation(b"nfs", libc::MNT_LOCAL as u32).is_err());
        assert!(apfs_observation(b"apfs", 0).is_err());
        assert!(apfs_observation(b"apfs", (libc::MNT_LOCAL | libc::MNT_RDONLY) as u32).is_err());
    }
}
