# Native APFS protocol feasibility

Observed on 2 November 2026 with installed Rust 1.99.1, macOS 26.6.2
build 25G83, arm64. [native.json](native.json) binds the exact probe source,
compiler, executable and [run log](native.log). The isolated fixture is on
local writable APFS, `/dev/disk3s5`, mounted at `/System/Volumes/Data`;
descriptor-bound `fstatfs` and `fpathconf` report a case-insensitive volume.
The repository and fixture use the same observed volume. `diskutil` initially
failed in the sandbox and did not resolve the repository path outside it;
the explicit Data mount resolved outside the sandbox. Admission relies on
descriptor observations, not that path lookup.

The installed standard-library source implements Apple `sync_all` with
`fcntl(F_FULLFSYNC)`; its exact source digest and excerpt are recorded.
The selected adapter mechanism will explicitly call `F_FULLFSYNC` for regular
files and `fsync` for directories, with no weaker fallback. The probe exercised
both the explicit full flush and installed Rust regular-file sync successfully.
It installed a hard link, persisted its destination, removed staging and persisted
that source directory, then flushed the retained file. It replaced old/new
references across staging/reference directories, synced both directories and
full-flushed the published regular reference before checking exact bytes.
All calls succeeded. Invalid descriptors returned native EBADF (9), and exclusive
file/directory creation against existing names returned EEXIST (17).

Stable nonblocking `flock` was exercised with independently opened same-process
descriptors and child processes. Shared locks coexist; exclusive attempts are
busy against readers; a child exclusive owner excludes both modes. SIGKILL and
wait release child ownership. Reacquisition leaves the lock inode unchanged.
The probe never replaces the lock. Host restrictions remain coordinated writers
on one host/volume, ordinary managed directories, and no external or cloud-sync
writers. Filesystem detection cannot establish the absence of those writers.

These observations establish the mechanism feasibility gate for implementation.
They do not establish the full library recovery matrix, Linux regression,
cross-host continuity, power-loss/kernel-crash/hardware durability or acceptance.
[retained-before.json](retained-before.json) hashes existing canonical files and
historical evidence without invoking a store decoder. `old_hub` is absent on this
checkout; no retained store was created or opened.
