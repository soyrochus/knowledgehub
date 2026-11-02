"""Required native observations; missing facts fail rather than inventing support evidence."""
import json
import pathlib
import platform
import plistlib
import subprocess


def observations(path, system=None, machine=None, run=subprocess.check_output):
    system = system or platform.system()
    machine = machine or platform.machine()
    path = pathlib.Path(path).resolve()
    compiler = run(["rustc", "-Vv"], text=True)
    if "release: 1.99.1\n" not in compiler:
        raise ValueError("native verification requires installed Rust 1.99.1")
    if system == "Linux" and machine == "x86_64":
        mount = json.loads(run(["findmnt", "-T", str(path), "-J", "-o", "TARGET,SOURCE,FSTYPE,OPTIONS"], text=True))
        entries = mount.get("filesystems", [])
        if len(entries) != 1 or entries[0].get("fstype") != "ext4" or not all(entries[0].get(k) for k in ("target", "source", "options")):
            raise ValueError("required actual Linux/ext4 mount facts are missing or incompatible")
        return dict(system=system, architecture=machine, compiler=compiler, profile="linux-x86_64/local-ext4/single-host/1", mount=mount,
                    kernel=run(["uname", "-a"], text=True).strip())
    if system == "Darwin" and machine == "arm64":
        os_version = run(["sw_vers", "-productVersion"], text=True).strip()
        os_build = run(["sw_vers", "-buildVersion"], text=True).strip()
        # diskutil resolves the actual mount, including APFS firmlink destinations,
        # more reliably than a nested directory path.
        df = run(["df", "-P", str(path)], text=True).splitlines()
        if len(df) != 2 or len(df[1].split()) < 6:
            raise ValueError("required Mac mount observation is missing")
        mountpoint = df[1].split(None, 5)[5]
        volume = plistlib.loads(run(["diskutil", "info", "-plist", mountpoint]))
        if not os_version or not os_build or volume.get("FilesystemType") != "apfs" or volume.get("MountPoint") != mountpoint or volume.get("WritableVolume") is not True or not volume.get("DeviceNode") or not volume.get("VolumeUUID"):
            raise ValueError("required writable Mac/APFS volume facts are missing or incompatible")
        return dict(system="macOS", architecture=machine, compiler=compiler, os_version=os_version, os_build=os_build,
                    profile="macos-aarch64/local-apfs/single-host/1", volume=volume,
                    kernel=run(["uname", "-a"], text=True).strip())
    raise ValueError("unsupported native verification host: " + system + "/" + machine)
