# Native local file profiles

The adapter owns platform mechanisms; `KnowledgeHub` retains one full semantic
API and files/4 format. Use `FileOptions::native_local()?` to compose a declaration,
or `local_ext4()` / `local_apfs()` explicitly. This selects a host assumption;
create/open independently check the path. Supported declarations are coordinated
Linux x86_64/local ext4 and macOS arm64/writable local APFS. The host must exclude
external/cloud-sync writers and simultaneous multi-host use.

Linux checks descriptor filesystem magic and the actual device's ext4 mount entry
in `/proc/self/mountinfo` (ext2/ext3 share the magic). Mac checks descriptor-bound
`fstatfs` for `apfs`, MNT_LOCAL and absence of MNT_RDONLY. Common checks reject
symlink ancestors/managed directories and cross-device layouts. Injected APFS
name/flag and device-ID negatives are tests of refusal logic, not observations of
a mounted NFS/read-only/cross-device fixture. Native tests exercise wrong host
declarations, uncoordinated declarations and actual symlink layouts.

`hub.json.filesystem_profile` records creation provenance, independently of the
current location. Exactly `linux-x86_64/local-ext4/single-host/1` and
`macos-aarch64/local-apfs/single-host/1` are recognised. Linux metadata shape,
canonical encoding, codecs and profile label remain unchanged. Open never rewrites
the label; unknown labels/codecs/formats/scope fail before mutation. A pre-extension
Linux binary may refuse a Mac-created profile. Synthetic label tests establish
parser/history identity, not actual cross-host transfer. Complete closed fixtures
still require both native copy directions before continuity is verified.

Linux retains regular-file/directory `sync_all` and the existing ordering/error
boundary. Mac uses checked `fcntl(F_FULLFSYNC)` for regular files, native `fsync`
for directories, and another full-file flush after namespace persistence. There
is no fallback to a weaker successful operation. The selected protocol was first
exercised by the [native feasibility probe](../delivery/evidence/macos-file-backend/feasibility/README.md).

For ordinary publication, persist candidate files and all installation directories,
persist the temporary reference, atomically replace the same-directory head,
sync its directory, then full-flush the published reference before `head-synced`
and acknowledgement. Recovery syncs the reference and its directory, then fences
the reference before returning outcomes. Creation finishes root/parent namespace
persistence with a regular metadata-file fence. Before compaction deletion,
install and verify the replacement pack, sidecar and catalogue, persist their
directories and finish a retained-pack full fence. Cleanup syncs removal parents
and finishes with that retained pack. Both derived caches use the same helpers
and final manifest fence; failed derived preparation stays an explicit index error.

All direct `sync_all` calls are confined to the Linux native helpers. Stable
descriptor-owned nonblocking `flock`, shared read leases, exclusive writers and
maintenance, original retry identity, finite quotas and the eight publication /
seven maintenance fault checkpoints remain. New injected file, directory and Mac
fence failures check unchanged accepted state before replacement and uncertainty
with exact original recovery after replacement. These are injected errors and
process-fault tests, not power-loss/kernel-crash/hardware evidence.

The native verifier records required OS/build/architecture/compiler/mount facts;
missing facts produce a failure log without a successful manifest. Mac uses
`df -P` to resolve the mount then `diskutil info -plist`; Linux retains `findmnt`.
Host scripts require Python 3.11 or later. Benchmarks report optional peak resident
bytes: Linux VmHWM KiB is multiplied by 1024, Darwin `getrusage.ru_maxrss` already
uses bytes. Unavailable/invalid metrics are JSON null. Encoded/allocated file bytes
are actual measurements; APFS allocation is not inferred from ext4 results.

Use isolated absolute unused store paths. Never invoke canonical reset to verify
support. Historical captures and repository definition ownership remain exact.
See [current support evidence](../delivery/evidence/macos-file-backend/README.md)
for observed results and outstanding native evidence.
