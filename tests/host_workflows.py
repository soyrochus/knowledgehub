"""Disposable host reset/checkpoint tests. Never open the real canonical Hub."""
import contextlib
import io
import json
import pathlib
import sys
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
sys.dont_write_bytecode = True
import host_sources as sources
import rebuild_canonical_hub as rebuild
import verify_full_ontology as native


class HostWorkflows(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(dir=ROOT / "target")
        self.root = pathlib.Path(self.temporary.name)
        (self.root / "src").mkdir()
        (self.root / "src/lib.rs").write_text("// fixture\n")
        for name in ("Cargo.toml", "Cargo.lock", "README.md", ".gitignore"):
            (self.root / name).write_text("fixture\n")
        self.hub = self.root / "hub"
        self.attempt = 0

    def tearDown(self):
        self.temporary.cleanup()

    def old(self, scope=None):
        self.hub.mkdir()
        (self.hub / "hub.json").write_text(json.dumps({"format": "knowledgehub-files/2", "scope": scope or rebuild.SCOPE}))
        (self.hub / "branch.lock").touch()
        (self.hub / "old-history").write_text("sentinel")

    def run_attempt(self, preflight=lambda: None, populate=None, verify=lambda: {"verified": True}):
        self.attempt += 1
        def default_population():
            self.assertFalse(self.hub.exists())
            self.hub.mkdir()
            (self.hub / "new-generation").write_text("fresh")
        with contextlib.redirect_stdout(io.StringIO()):
            return rebuild.lifecycle(self.root, self.hub, self.root / f"run-{self.attempt}", preflight, populate or default_population, verify)

    def test_reset_removes_old_content_and_preserves_external_evidence(self):
        self.old()
        evidence = self.root / "historical-evidence"
        evidence.write_bytes(b"older run stays exact")
        status = self.run_attempt()
        self.assertEqual(status["state"], "verified")
        self.assertFalse((self.hub / "old-history").exists())
        self.assertEqual(evidence.read_bytes(), b"older run stays exact")

    def test_unsafe_parent_repository_unrelated_and_symlink_targets(self):
        self.old()
        other = self.root / "unrelated"
        other.mkdir()
        (other / "sentinel").write_text("keep")
        for target in (self.root, self.root.parent, other, self.root / ".." / self.root.name / "hub"):
            with self.subTest(target=target), self.assertRaises(ValueError):
                rebuild.target_info(self.root, target)
        import shutil
        shutil.rmtree(self.hub)
        self.hub.symlink_to(other, target_is_directory=True)
        with self.assertRaises(ValueError):
            rebuild.target_info(self.root, self.hub)
        self.assertEqual((other / "sentinel").read_text(), "keep")

    def test_subtree_symlinks_and_unexpected_scope_are_refused(self):
        self.old()
        (self.hub / "escape").symlink_to(self.root)
        with self.assertRaises(ValueError):
            rebuild.target_info(self.root, self.hub)
        (self.hub / "escape").unlink()
        data = json.loads((self.hub / "hub.json").read_text())
        data["scope"]["owner"] = "unrelated-owner"
        (self.hub / "hub.json").write_text(json.dumps(data))
        with self.assertRaises(ValueError):
            rebuild.target_info(self.root, self.hub)
        self.assertTrue((self.hub / "old-history").exists())

    def test_failed_preflight_preserves_the_old_hub(self):
        self.old()
        def fail():
            raise RuntimeError("a required check failed")
        with self.assertRaises(RuntimeError):
            self.run_attempt(preflight=fail)
        status = json.loads((self.root / "run-1/status.json").read_text())
        self.assertEqual(status["state"], "failed")
        self.assertFalse(status["reset_started"])
        self.assertEqual((self.hub / "old-history").read_text(), "sentinel")

    def test_active_backend_writer_prevents_deletion(self):
        self.old()
        with rebuild.exclusive(self.hub / "branch.lock"), self.assertRaises(BlockingIOError):
            self.run_attempt()
        self.assertTrue((self.hub / "old-history").exists())

    def test_interrupted_population_is_failed_and_explicit_retry_is_fresh(self):
        self.old()
        def partial():
            self.hub.mkdir()
            (self.hub / "partial").write_text("unfinished")
            raise InterruptedError("interrupted after reset")
        with self.assertRaises(InterruptedError):
            self.run_attempt(populate=partial)
        self.assertEqual(json.loads((self.root / "run-1/status.json").read_text())["state"], "failed")
        status = self.run_attempt()
        self.assertEqual(status["state"], "verified")
        self.assertFalse((self.hub / "partial").exists())
        self.assertTrue((self.root / "run-1/status.json").exists())

    def test_unmarked_partial_directory_is_not_a_reset_target(self):
        self.hub.mkdir()
        (self.hub / "unrelated").write_text("keep")
        with self.assertRaises(ValueError):
            rebuild.target_info(self.root, self.hub)

    def test_failed_dataset_verification_never_marks_the_new_store_verified(self):
        self.old()
        def fail():
            raise ValueError("scrub or indexed read failed")
        with self.assertRaises(ValueError):
            self.run_attempt(verify=fail)
        status=json.loads((self.root/"run-1/status.json").read_text())
        self.assertEqual(status["state"],"failed")
        self.assertTrue(status["reset_started"])
        self.assertTrue((self.hub/"new-generation").exists())

    def test_outputs_cache_and_evidence_cannot_recurse_into_capture(self):
        for group in ("hub", "orig_hub", "target", "delivery/evidence/old", "src/cache", "examples/__pycache__"):
            directory = self.root / group
            directory.mkdir(parents=True, exist_ok=True)
            (directory / "recursive.json").write_text("{}")
        paths = sources.checkpoint(self.root)
        self.assertFalse(any("recursive.json" in path for path in paths))
        self.assertIn("src/lib.rs", paths)

    def test_stale_added_removed_source_and_log_or_build_bytes_are_refused(self):
        directory = self.root / "run"
        directory.mkdir()
        log = directory / "native.log"
        log.write_text("passed")
        binary = self.root / "target/checked-binary"
        binary.parent.mkdir(exist_ok=True)
        binary.write_bytes(b"exact executable")
        data = dict(format="knowledgehub-observed-native-run/2", files=sources.checkpoint(self.root), log=sources.digest(log.read_bytes()), commands=[dict(command=c,exit_code=0) for c in native.COMMANDS], executables={"target/checked-binary": sources.digest(binary.read_bytes())})
        manifest = directory / "checked.json"
        manifest.write_text(json.dumps(data))
        sources.checked_run(self.root, manifest, log)
        for path, replacement in [(self.root / "src/lib.rs", b"changed source"), (log, b"stale log"), (binary, b"other executable")]:
            original = path.read_bytes()
            path.write_bytes(replacement)
            with self.assertRaises(ValueError):
                sources.checked_run(self.root, manifest, log)
            path.write_bytes(original)
        added = self.root / "src/new.rs"
        added.write_text("new source")
        with self.assertRaises(ValueError):
            sources.checked_run(self.root, manifest, log)
        added.unlink()
        original=(self.root/"src/lib.rs").read_bytes()
        (self.root/"src/lib.rs").unlink()
        with self.assertRaises(ValueError):
            sources.checked_run(self.root,manifest,log)
        (self.root/"src/lib.rs").write_bytes(original)

    def test_historical_native_evidence_is_not_overwritten(self):
        directory = self.root / "run"
        directory.mkdir()
        (directory / "native.log").write_text("older exact log")
        with self.assertRaises(ValueError):
            native.verify(self.root, directory)
        self.assertEqual((directory / "native.log").read_text(), "older exact log")


if __name__ == "__main__":
    unittest.main()
