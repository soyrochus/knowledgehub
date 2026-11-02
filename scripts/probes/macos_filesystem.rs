//! Native feasibility probe; run only against a newly created disposable APFS directory.
use serde_json::json;
use std::{
    ffi::CStr,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Write},
    os::{fd::AsRawFd, unix::fs::MetadataExt},
    path::Path,
    process::{Command, Stdio},
};

fn syscall(result: i32) -> io::Result<()> {
    if result == 0 { Ok(()) } else { Err(io::Error::last_os_error()) }
}
fn flush(file: &File) -> io::Result<()> {
    // SAFETY: file owns a live descriptor; F_FULLFSYNC takes no pointer argument.
    syscall(unsafe { libc::fcntl(file.as_raw_fd(), libc::F_FULLFSYNC) })
}
fn directory(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    // SAFETY: file owns a live directory descriptor.
    syscall(unsafe { libc::fsync(file.as_raw_fd()) })
}
fn lock(file: &File, mode: i32) -> io::Result<()> {
    // SAFETY: descriptor is owned; every attempt is nonblocking.
    syscall(unsafe { libc::flock(file.as_raw_fd(), mode | libc::LOCK_NB) })
}
fn opened(path: &Path) -> File {
    OpenOptions::new().read(true).write(true).open(path).unwrap()
}
fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args[1] == "--child" {
        let file = opened(Path::new(&args[2]));
        let mode = if args[3] == "shared" { libc::LOCK_SH } else { libc::LOCK_EX };
        match lock(&file, mode) {
            Ok(()) => {
                println!("locked");
                io::stdout().flush().unwrap();
                // Parent kills this process; the descriptor and lock must be released.
                std::thread::park();
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => std::process::exit(73),
            Err(error) => panic!("child lock: {error}"),
        }
        return;
    }
    let root = Path::new(&args[1]);
    fs::create_dir(root).unwrap(); // Exclusive, never use an existing store.
    let descriptor = File::open(root).unwrap();
    let mut stat = std::mem::MaybeUninit::<libc::statfs>::uninit();
    // SAFETY: writable stat storage and a live descriptor.
    syscall(unsafe { libc::fstatfs(descriptor.as_raw_fd(), stat.as_mut_ptr()) }).unwrap();
    // SAFETY: successful fstatfs initialized stat and its NUL-terminated names.
    let stat = unsafe { stat.assume_init() };
    let filesystem = unsafe { CStr::from_ptr(stat.f_fstypename.as_ptr()) }.to_str().unwrap();
    assert_eq!(filesystem, "apfs");
    assert_ne!(stat.f_flags & libc::MNT_LOCAL as u32, 0);
    assert_eq!(stat.f_flags & libc::MNT_RDONLY as u32, 0);
    // SAFETY: live descriptor, documented Darwin pathconf selector.
    let case_sensitive = unsafe { libc::fpathconf(descriptor.as_raw_fd(), libc::_PC_CASE_SENSITIVE) };
    assert!(case_sensitive == 0 || case_sensitive == 1);
    let mount = unsafe { CStr::from_ptr(stat.f_mntonname.as_ptr()) }.to_str().unwrap();
    let source = unsafe { CStr::from_ptr(stat.f_mntfromname.as_ptr()) }.to_str().unwrap();
    let mut steps = Vec::new();
    for name in ["staging", "objects", "refs"] {
        fs::create_dir(root.join(name)).unwrap();
        directory(&root.join(name)).unwrap();
    }
    directory(root).unwrap();
    steps.push("create directories; fsync children and parent");
    let object = root.join("staging/object");
    let mut file = OpenOptions::new().write(true).create_new(true).open(&object).unwrap();
    file.write_all(b"exact immutable candidate").unwrap();
    flush(&file).unwrap();
    file.sync_all().unwrap(); // Installed Rust primitive also succeeds, independently of explicit call.
    steps.push("write candidate; explicit F_FULLFSYNC; installed Rust sync_all");
    fs::hard_link(&object, root.join("objects/retained")).unwrap();
    directory(&root.join("objects")).unwrap();
    fs::remove_file(&object).unwrap();
    directory(&root.join("staging")).unwrap();
    flush(&File::open(root.join("objects/retained")).unwrap()).unwrap();
    assert_eq!(fs::read(root.join("objects/retained")).unwrap(), b"exact immutable candidate");
    steps.push("hard-link install; fsync destination; unlink staging; fsync source; retained-file full fence");
    for bytes in [b"old".as_slice(), b"complete new".as_slice()] {
        let path = root.join("staging/reference");
        let mut reference = OpenOptions::new().write(true).create_new(true).open(&path).unwrap();
        reference.write_all(bytes).unwrap();
        flush(&reference).unwrap();
        directory(&root.join("staging")).unwrap();
        fs::rename(&path, root.join("refs/head")).unwrap();
        directory(&root.join("refs")).unwrap();
        directory(&root.join("staging")).unwrap();
        flush(&File::open(root.join("refs/head")).unwrap()).unwrap();
        assert_eq!(fs::read(root.join("refs/head")).unwrap(), bytes);
    }
    steps.push("write/full-flush reference; fsync staging; same-volume replace; fsync both parents; published-file full fence");
    let creation_error = OpenOptions::new().write(true).create_new(true).open(root.join("refs/head")).unwrap_err();
    assert_eq!(creation_error.kind(), io::ErrorKind::AlreadyExists);
    let mkdir_error = fs::create_dir(root).unwrap_err();
    assert_eq!(mkdir_error.kind(), io::ErrorKind::AlreadyExists);
    // Native errors, not simulated persistence failures.
    let bad_flush = syscall(unsafe { libc::fcntl(-1, libc::F_FULLFSYNC) }).unwrap_err();
    let bad_sync = syscall(unsafe { libc::fsync(-1) }).unwrap_err();
    assert_eq!(bad_flush.raw_os_error(), Some(libc::EBADF));
    assert_eq!(bad_sync.raw_os_error(), Some(libc::EBADF));

    let lock_path = root.join("stable.lock");
    File::create_new(&lock_path).unwrap();
    let inode = lock_path.metadata().unwrap().ino();
    let first = opened(&lock_path);
    let second = opened(&lock_path);
    lock(&first, libc::LOCK_SH).unwrap();
    lock(&second, libc::LOCK_SH).unwrap();
    assert_eq!(lock(&opened(&lock_path), libc::LOCK_EX).unwrap_err().kind(), io::ErrorKind::WouldBlock);
    let exe = std::env::current_exe().unwrap();
    let status = Command::new(&exe).arg("--child").arg(&lock_path).arg("exclusive").status().unwrap();
    assert_eq!(status.code(), Some(73));
    let spawn = |mode| Command::new(&exe).arg("--child").arg(&lock_path).arg(mode).stdout(Stdio::piped()).spawn().unwrap();
    let mut child = spawn("shared");
    let mut line = String::new();
    io::BufReader::new(child.stdout.take().unwrap()).read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "locked");
    child.kill().unwrap(); child.wait().unwrap();
    drop(first); drop(second);
    lock(&opened(&lock_path), libc::LOCK_EX).unwrap();
    let mut child = spawn("exclusive");
    let mut line = String::new();
    io::BufReader::new(child.stdout.take().unwrap()).read_line(&mut line).unwrap();
    assert_eq!(line.trim(), "locked");
    assert_eq!(lock(&opened(&lock_path), libc::LOCK_SH).unwrap_err().kind(), io::ErrorKind::WouldBlock);
    assert_eq!(lock(&opened(&lock_path), libc::LOCK_EX).unwrap_err().kind(), io::ErrorKind::WouldBlock);
    child.kill().unwrap(); child.wait().unwrap();
    let exclusive = opened(&lock_path);
    lock(&exclusive, libc::LOCK_EX).unwrap();
    assert_eq!(lock(&opened(&lock_path), libc::LOCK_EX).unwrap_err().kind(), io::ErrorKind::WouldBlock);
    assert_eq!(lock_path.metadata().unwrap().ino(), inode);
    drop(exclusive);
    println!("{}", json!({
        "format":"knowledgehub-apfs-feasibility/1", "filesystem":filesystem,
        "mount":mount, "source":source, "flags":stat.f_flags, "case_sensitive":case_sensitive == 1,
        "device":descriptor.metadata().unwrap().dev(), "steps":steps,
        "errors":{"exclusive_file":creation_error.raw_os_error(),"exclusive_directory":mkdir_error.raw_os_error(),"invalid_flush":bad_flush.raw_os_error(),"invalid_fsync":bad_sync.raw_os_error()},
        "locking":"same-process independent shared/exclusive; child shared/exclusive; busy exit 73; SIGKILL release; unchanged inode",
        "limits":"native successful calls and process coordination; no power-loss/kernel-crash/hardware guarantee"
    }));
}
