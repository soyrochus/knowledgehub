#!/usr/bin/env python3
"""Execute a fresh complete native batch and bind exact source, logs and binaries."""
import argparse
import datetime
import json
import pathlib
import os
import re
import subprocess
import sys
sys.dont_write_bytecode = True
from host_sources import checkpoint, digest, require_checkpoint, package_identity
from host_platform import observations

ROOT = pathlib.Path(__file__).resolve().parents[1]
COMMANDS = [
    ["cargo", "fmt", "--all", "--", "--check"],
    ["cargo", "test", "--offline", "--all-targets"],
    ["cargo", "test", "--offline", "--doc"],
    ["cargo", "clippy", "--offline", "--all-targets", "--", "-D", "warnings"],
    [sys.executable, "-m", "unittest", "discover", "-s", "tests", "-p", "host_workflows.py", "-v"],
]


def verify(root, directory):
    directory.mkdir(parents=True, exist_ok=True)
    output, manifest = directory / "native.log", directory / "checked.json"
    if output.exists() or manifest.exists():
        raise ValueError("fresh evidence paths required; older native evidence is never overwritten")
    version = package_identity(root)
    files = checkpoint(root)
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    results = []
    with output.open("xb") as log:
        try:
            platform_facts = observations(root)
        except (ValueError, OSError, subprocess.CalledProcessError) as error:
            log.write(("Required native observations failed: " + str(error) + "\n").encode())
            raise ValueError("required native observations failed; see " + str(output)) from error
        log.write((json.dumps(platform_facts, indent=2) + "\n").encode())
        for command in [["rustc", "-Vv"], *COMMANDS]:
            log.write(("\n$ " + " ".join(command) + "\n").encode())
            log.flush()
            print("$ " + " ".join(command), flush=True)
            result = subprocess.run(command, cwd=root, stdout=log, stderr=subprocess.STDOUT, env={**os.environ, "RUST_TEST_THREADS": "4"})
            results.append({"command": command, "exit_code": result.returncode, "environment": {"RUST_TEST_THREADS": "4"}})
            if result.returncode:
                raise RuntimeError(f"required check failed ({result.returncode}): {command}; output: {output}")
    require_checkpoint(root, files)
    executables = {}
    for command in [["cargo", "test", "--offline", "--all-targets", "--no-run", "--message-format=json"], ["cargo", "build", "--offline", "--release", "--examples", "--message-format=json"]]:
        result = subprocess.run(command, cwd=root, capture_output=True, text=True, check=True)
        metadata = directory / ("release-build.log" if "--release" in command else "test-build.log")
        metadata.write_text(result.stdout + result.stderr)
        results.append({"command": command, "exit_code": result.returncode, "log": digest(metadata.read_bytes())})
        for line in result.stdout.splitlines():
            try:
                item = json.loads(line)
            except json.JSONDecodeError:
                continue
            if item.get("executable"):
                path = pathlib.Path(item["executable"])
                executables[str(path.relative_to(root))] = digest(path.read_bytes())
    require_checkpoint(root, files)
    counts = [int(n) for n in re.findall(r"test result: ok\. (\d+) passed", output.read_text())]
    coverage_path = root / "delivery/evidence/v010/coverage-map.json"
    coverage = json.loads(coverage_path.read_text())
    coverage["guarantees"].update({
        "Native declarations and injected device-profile refusal": ["declarations_are_explicit_and_coordination_is_required", "native_profile_refuses_wrong_host_and_symlink_layout_before_mutation"],
        "Recognised creation provenance, exact history and metadata refusal": ["native_creation_profiles_are_provenance_and_unknown_metadata_never_mutates"],
        "Native sync/fence prior-or-complete-new recovery": ["native_sync_failures_preserve_prior_or_recover_exact_original_receipt"],
        "Native benchmark RSS units and unavailable values": ["rss_units_missing_and_overflow_are_explicit"],
    })
    if platform_facts["system"] == "macOS":
        coverage["guarantees"]["Injected APFS nonlocal/readonly/filesystem refusal"] = ["injected_apfs_observations_refuse_nonlocal_readonly_and_other_filesystems"]
    passed = set(re.findall(r"^test ([\w:]+) \.\.\. ok$", output.read_text(), re.M))
    for guarantee, cases in coverage["guarantees"].items():
        for case in cases:
            if not any(name == case or name.endswith("::" + case) for name in passed):
                raise ValueError("unobserved current safety case: " + guarantee + ": " + case)
    data = dict(format="knowledgehub-observed-native-run/2", library_version=version, started_at=started,
                ended_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                source_status="exact working tree before capture/final task and generated-report updates",
                files=files, executables=executables, commands=results, log=digest(output.read_bytes()),
                native_test_counts=counts, platform=platform_facts, guarantee_coverage=coverage, coverage_map=digest(coverage_path.read_bytes()),
                limits=["host-declared coordinated local profile; actual platform observations in log", "no power-loss, remote-adapter, acceptance or authority-handover claim"])
    manifest.write_text(json.dumps(data, indent=2) + "\n")
    return manifest


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-dir", required=True, help="new run-specific evidence directory")
    args = parser.parse_args()
    manifest = verify(ROOT, ROOT / args.run_dir)
    print(f"Complete native batch passed: {manifest}", flush=True)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, RuntimeError, subprocess.CalledProcessError) as error:
        print(error, file=sys.stderr)
        sys.exit(1)
