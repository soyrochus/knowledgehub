#!/usr/bin/env python3
"""Explicit host lifecycle for the disposable repository-root canonical demonstration."""
import argparse
import contextlib
import datetime
import fcntl
import hashlib
import json
import os
import pathlib
import shutil
import stat
import subprocess
import sys
import tempfile
import time
import uuid
os.environ["PYTHONDONTWRITEBYTECODE"] = "1"
sys.dont_write_bytecode = True
from host_sources import checked_run, digest, require_checkpoint

ROOT = pathlib.Path(__file__).resolve().parents[1]
SCOPE = dict(hub="kh:hub:self-description", model="urn:knowledgehub:library", branch="main", owner="host:maintainer")
SCHEMA = "knowledgehub-canonical-rebuild/1"


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write_json(path, data):
    temporary = path.with_suffix(path.suffix + ".tmp")
    with temporary.open("w") as stream:
        json.dump(data, stream, indent=2)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(temporary, path)


def marker(root):
    path = root / ".hub-rebuild-state.json"
    if path.is_symlink():
        raise ValueError("rebuild status cannot be a symlink")
    return json.loads(path.read_text()) if path.exists() else {}


def target_info(root, target):
    root = root.resolve(strict=True)
    if ".." in target.parts or target != root / "hub" or target.is_symlink():
        raise ValueError("only the exact ordinary repository-root hub/ is a reset target")
    if not (root / "Cargo.toml").is_file() or not (root / "src/lib.rs").is_file():
        raise ValueError("expected repository boundaries")
    if not target.exists():
        return dict(exists=False, files=0, bytes=0, replay_domain=None)
    mode = target.lstat()
    if not stat.S_ISDIR(mode.st_mode):
        raise ValueError("canonical target is not an ordinary directory")
    count = size = 0
    for base, directories, files in os.walk(target, followlinks=False):
        for name in directories + files:
            path = pathlib.Path(base) / name
            entry = path.lstat()
            if path.is_symlink() or entry.st_dev != mode.st_dev or not (stat.S_ISDIR(entry.st_mode) or stat.S_ISREG(entry.st_mode)):
                raise ValueError("canonical subtree contains a link, foreign mount or nonordinary entry")
            if stat.S_ISREG(entry.st_mode):
                count += 1
                size += entry.st_size
    metadata = target / "hub.json"
    if metadata.exists():
        if metadata.stat().st_size > 1048576:
            raise ValueError("unexpected Hub metadata size")
        data = json.loads(metadata.read_text())
        if data.get("format") not in ("knowledgehub-files/2", "knowledgehub-files/3") or any(data.get("scope", {}).get(k) != v for k, v in SCOPE.items()):
            raise ValueError("unexpected generated Hub scope/format")
    else:
        previous = marker(root)
        if previous.get("format") != SCHEMA or previous.get("target") != str(target) or previous.get("scope") != SCOPE or not (previous.get("reset_started") or previous.get("partial_scope_verified")):
            raise ValueError("unmarked directory without expected generated Hub metadata")
        data = {}
    return dict(exists=True, files=count, bytes=size, replay_domain=data.get("replay_domain"),
                format=data.get("format"), inode=mode.st_ino)


@contextlib.contextmanager
def exclusive(path):
    descriptor = os.open(path, os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600)
    try:
        if not stat.S_ISREG(os.fstat(descriptor).st_mode):
            raise ValueError("coordination path is not an ordinary lock")
        fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        yield
    finally:
        os.close(descriptor)


def reset(root, target, before):
    """Called only after checks/capture; uses the existing adapter creation/store locks."""
    creation = root / (".knowledgehub-create-" + hashlib.sha256(b"hub").hexdigest() + ".lock")
    with exclusive(creation), contextlib.ExitStack() as stack:
        if before["exists"]:
            lock = target / ("locks/store.lock" if before.get("format") == "knowledgehub-files/3" else "branch.lock")
            if before.get("format"):
                if not lock.is_file() or lock.is_symlink():
                    raise ValueError("expected stable backend lock")
                stack.enter_context(exclusive(lock))
            current = target_info(root, target)
            if current.get("inode") != before.get("inode"):
                raise ValueError("canonical target changed while coordinating reset")
            shutil.rmtree(target)
        if target.exists() or target.is_symlink():
            raise ValueError("reset did not remove the complete target")
    # Release the adapter creation lock before the ordinary child create API acquires it.


def lifecycle(root, target, directory, preflight, populate, verify):
    """Injectable phases permit disposable tests; the CLI always supplies the real full batch."""
    directory.mkdir(parents=True, exist_ok=True)
    status_path = directory / "status.json"
    if status_path.exists():
        raise ValueError("use a new attempt directory; previous evidence is retained")
    target = pathlib.Path(target)
    before = target_info(root, target)
    status = dict(format=SCHEMA, state="preflight", target=str(target), scope=SCOPE,
                  started_at=now(), reset_started=False, before=before,
                  partial_scope_verified=before["exists"] and before.get("format") is None)
    def phase(state, **extra):
        status.update(state=state, updated_at=now(), **extra)
        write_json(status_path, status)
        write_json(root / ".hub-rebuild-state.json", status)
        print("canonical rebuild: " + state, flush=True)
    with exclusive(root / ".hub-rebuild.lock"):
        try:
            phase("preflight")
            preflight()
            phase("checks-passed")
            # Refuse a concurrently changed scope/path before recording reset intent.
            current = target_info(root, target)
            if current != before:
                raise ValueError("canonical target changed after preflight")
            phase("reset-started", reset_started=True)
            reset(root, target, before)
            populate()
            phase("populated")
            result = verify()
            phase("verified", completed_at=now(), result=result)
            return status
        except BaseException as error:
            phase("failed", failed_at=now(), error=type(error).__name__ + ": " + str(error))
            raise


def command(root, directory, label, args, results):
    output, errors = directory / (label + ".json"), directory / (label + ".stderr.log")
    started = time.monotonic()
    with output.open("xb") as out, errors.open("xb") as err:
        result = subprocess.run([str(x) for x in args], cwd=root, stdout=out, stderr=err)
    item = dict(name=label, command=[str(x) for x in args], exit_code=result.returncode,
                seconds=time.monotonic() - started, output=digest(output.read_bytes()), stderr=digest(errors.read_bytes()))
    results.append(item)
    write_json(directory / "commands.json", results)
    if result.returncode:
        raise RuntimeError(f"{label} failed; see {errors}")
    return output


def packed_cache_report(target, inspection):
    """Host physical measurements; accepted data is inspected through the public API."""
    report = {}
    for name, quota, counter in (("neighbours", 256 << 20, "neighbour"), ("text", 128 << 20, "text")):
        root = target / "cache" / name
        files = sorted(p for p in root.rglob("*") if p.is_file())
        packs = sorted((root / "objects/packs").glob("*.khpack"))
        if not packs or any((root / "objects/loose").rglob("*.obj")):
            raise ValueError("fresh derived cache must use sealed packs: " + name)
        bindings = {str(p.relative_to(target)): digest(p.read_bytes()) for p in packs}
        if any(binding["sha256"] != pathlib.Path(p).stem for p, binding in bindings.items()):
            raise ValueError("derived pack filename/hash mismatch: " + name)
        seen, allocated = set(), 0
        for path in files:
            m = path.stat()
            identity = (m.st_dev, m.st_ino)
            if identity not in seen:
                allocated += m.st_blocks * 512
                seen.add(identity)
        peak = inspection["indexed"]["adapter_work"][counter + "_cache_peak_allocated_bytes"]
        if allocated > quota or peak > quota:
            raise ValueError("derived allocation quota exceeded: " + name)
        nodes = set()
        sidecars = sorted((root / "objects/packs").glob("*.idx"))
        for path in sidecars:
            nodes.update(json.loads(path.read_text()))
        report[name] = dict(files=len(files), packs=len(packs), unique_objects=len(nodes),
                            encoded_bytes=sum(p.stat().st_size for p in files), allocated_bytes=allocated,
                            peak_reserved_bytes=peak, quota_bytes=quota, sealed_packs=bindings,
                            pack_bytes=sum(p.stat().st_size for p in packs),
                            sidecar_bytes=sum(p.stat().st_size for p in sidecars))
    return report


def verify_reopened(first, second):
    for key in ("head", "baseline_a", "baseline_b", "original_receipts", "changed_identities"):
        if first[key] != second[key]:
            raise ValueError("reopened canonical result differs: " + key)
    for key in ("warm_neighbours", "search", "composite"):
        semantic = lambda value: {k: v for k, v in value.items() if k not in ("work", "continuation")}
        if semantic(first["indexed"][key]) != semantic(second["indexed"][key]):
            raise ValueError("reopened indexed result differs: " + key)
    indexed = second["indexed"]
    if not indexed["cold_neighbours"]["ready"]:
        raise ValueError("reopened neighbour cache was not ready")
    cold = indexed["cold_neighbours"]["work"]["index_metadata_bytes_read"]
    text = indexed["search"]["work"]["index_metadata_bytes_read"]
    warm = [indexed["warm_neighbours"]["work"], indexed["composite"]["work"], indexed["repeated_composite_work"]]
    if cold == 0 or text == 0 or any(work["index_metadata_bytes_read"] != 0 for work in warm):
        raise ValueError("expected accounted cold metadata and reusable warm locations")
    return dict(exact_semantic_results=True, cold_neighbour_metadata_bytes=cold,
                cold_text_metadata_bytes=text, warm_metadata_bytes=0,
                first_composite_work=first["indexed"]["composite"]["work"],
                reopened_composite_work=indexed["composite"]["work"],
                composite_full_state_loads=indexed["composite_full_state_loads"])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--reset", action="store_true", help="explicitly discard this generated Hub instance")
    parser.add_argument("--target", type=pathlib.Path, default=ROOT / "hub")
    parser.add_argument("--run-dir", type=pathlib.Path)
    args = parser.parse_args()
    if not args.reset:
        parser.error("explicit --reset is required")
    directory = args.run_dir or ROOT / "delivery/evidence/hub-rebuild" / (datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])
    directory = (ROOT / directory).absolute()
    relative = directory.relative_to(ROOT)
    if ".." in relative.parts or relative.parts[:3] != ("delivery", "evidence", "hub-rebuild") or len(relative.parts) != 4 or directory.exists():
        raise ValueError("use a new direct run directory under delivery/evidence/hub-rebuild/")
    results = []
    checked = {}
    binaries = ROOT / "target/release/examples"
    def preflight():
        subprocess.run([sys.executable, ROOT / "scripts/verify_full_ontology.py", "--run-dir", directory], cwd=ROOT, check=True)
        checked.update(checked_run(ROOT, directory / "checked.json", directory / "native.log"))
        capture_args = [sys.executable, ROOT / "examples/capture_self_model.py", "--output", str(relative / "self-model-input.json"),
                        "--derived", str(relative / "ontology-pdf-extracted.txt"), "--log", str(relative / "native.log"),
                        "--checked-manifest", str(relative / "checked.json")]
        command(ROOT, directory, "capture", capture_args, results)
        command(ROOT, directory, "validate-input", [binaries / "full_self_description", "--format", "files3", "--validate-input", directory / "self-model-input.json"], results)
        require_checkpoint(ROOT, checked["files"])
    def populate():
        command(ROOT, directory, "populate", [binaries / "full_self_description", "--format", "files3", directory / "self-model-input.json", args.target, "--result", directory / "self-model-result.json"], results)
        metadata = json.loads((args.target / "hub.json").read_text())
        if metadata["format"] != "knowledgehub-files/3" or metadata["replay_domain"] == marker(ROOT)["before"].get("replay_domain"):
            raise ValueError("expected a fresh files/3 replay domain")
    def verify():
        inspection = command(ROOT, directory, "inspect", [binaries / "inspect_self_model", "--format", "files3", "--verify", "--compact", args.target], results)
        report = json.loads(inspection.read_text())
        caches = packed_cache_report(args.target, report)
        reopened = command(ROOT, directory, "inspect-reopened", [binaries / "inspect_self_model", "--format", "files3", "--verify", args.target], results)
        reopened_report = json.loads(reopened.read_text())
        reads = verify_reopened(report, reopened_report)
        after = packed_cache_report(args.target, reopened_report)
        if any(caches[name]["sealed_packs"] != after[name]["sealed_packs"] for name in caches):
            raise ValueError("sealed packs changed during read-only reopen")
        # Fixtures live on the supported repository filesystem, never in the canonical store.
        fixture_root = pathlib.Path(tempfile.mkdtemp(prefix="kh-rebuild-", dir=ROOT / "target"))
        catalogue = command(ROOT, directory, "catalogue", [binaries / "catalogue_conformance", "--format", "files3", fixture_root / "catalogue"], results)
        for example in ("neighbours", "rent_impact"):
            command(ROOT, directory, example, [binaries / example, fixture_root / example], results)
        command(ROOT, directory, "coverage", [sys.executable, ROOT / "scripts/audit_full_ontology.py", catalogue, directory / "self-model-result.json", directory / "self-model-input.json",
                "--log", directory / "native.log", "--output", directory / "coverage-audit.json"], results)
        meta = json.loads((args.target / "hub.json").read_text())
        capture = json.loads((directory / "self-model-input.json").read_text())
        final = dict(format=SCHEMA, replay_domain=meta["replay_domain"], scope=meta["scope"], native=checked["log"],
                     captured_at=capture["captured_at"], capture=digest((directory / "self-model-input.json").read_bytes()),
                     selected_records=report["selected_records"], retained_revisions=report["retained_revisions"],
                     explicit_gaps=report["explicit_gaps"], scrub=report["scrub"],
                     head=report["head"], original_receipts=report["original_receipts"],
                     captured_files=len(capture["files"]), source_units=len(capture["units"]),
                     packed_caches=caches, reopened_reads=reads,
                     source_checkpoint="captured after successful exact-input batch, before final task/report/generated-guide edits",
                     human_acceptance=False, authority_handover=False, migration_continuity=False)
        guide = f"""# Canonical KnowledgeHub demonstration

Fresh files/3 instance. Replay domain: {meta['replay_domain']}.
Captured checkpoint: {capture['captured_at']}; exact sources, native runs and original receipts are retained in this Hub.

Read it through the public API:

~~~sh
cargo run --offline --release --example inspect_self_model -- --format files3
~~~

Full scrub and actual model neighbour/literal/composite reads:

~~~sh
cargo run --offline --release --example inspect_self_model -- --format files3 --verify
~~~

This run has {report['selected_records']} selected records and {report['explicit_gaps']} explicit draft gaps.
Technical verification creates no human acceptance or definition-authority transfer.
The captured checkpoint precedes final task/report and this generated guide; repository edits do not refresh it.

[Exact run evidence](../{relative.as_posix()}/result.json) records sizes, checks, A/B and new receipts.
Reset starts fresh history and requires explicit intent:

~~~sh
python3 scripts/rebuild_canonical_hub.py --reset
~~~

Do not edit accepted backend files. Use public change APIs to retain history within this instance.
Historical external evidence belongs to its original instance; no old receipts were imported.
"""
        (args.target / "README.md").write_text(guide)
        write_json(directory / "result.json", final)
        return final
    lifecycle(ROOT, args.target, directory, preflight, populate, verify)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, RuntimeError, OSError, subprocess.CalledProcessError) as error:
        print(error, file=sys.stderr)
        sys.exit(1)
