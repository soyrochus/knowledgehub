"""Exact host checkpoints shared by native verification, capture and reset preflight."""
import hashlib
import json

GROUPS = ("source", "design", "delivery", "openspec", "src", "tests", "examples", "scripts")
SUFFIXES = {".rs", ".md", ".pdf", ".toml", ".json", ".txt", ".py", ".yaml", ".yml"}
EXCLUDED = {"evidence", "hub", "orig_hub", "target", "cache", "__pycache__", ".git"}


def digest(data):
    return {"sha256": hashlib.sha256(data).hexdigest(), "length": len(data)}


def change_dir(root, name):
    """Resolve a unique change origin before or after OpenSpec archiving."""
    changes = root / "openspec/changes"
    matches = [p for p in [changes / name, *(changes / "archive").glob("????-??-??-" + name)] if p.is_dir()]
    if len(matches) != 1:
        raise ValueError(f"expected one active or archived change for {name}; found {len(matches)}")
    return matches[0]


def source_paths(root):
    """Enumerate declared source trees only; never recursively include output/evidence."""
    paths = {root / "Cargo.toml", root / "Cargo.lock", root / "README.md", root / ".gitignore"}
    for group in GROUPS:
        for path in (root / group).rglob("*"):
            relative = path.relative_to(root)
            if not path.is_file() or path.is_symlink() or EXCLUDED.intersection(relative.parts):
                continue
            if path.suffix in SUFFIXES or group == "tests" and "fixtures" in relative.parts:
                paths.add(path)
    return sorted(paths)


def checkpoint(root):
    return {str(p.relative_to(root)): digest(p.read_bytes()) for p in source_paths(root)}


def require_checkpoint(root, expected):
    actual = checkpoint(root)
    if actual != expected:
        changed = sorted(p for p in actual.keys() | expected.keys() if actual.get(p) != expected.get(p))
        raise ValueError("checked source set/bytes changed: " + ", ".join(changed))


def checked_run(root, manifest, log):
    data = json.loads(manifest.read_text())
    if not data.get("commands") or any(c["exit_code"] != 0 for c in data["commands"]):
        raise ValueError("native checks did not pass")
    if digest(log.read_bytes()) != data["log"]:
        raise ValueError("run log differs from observed checked manifest")
    if data.get("format") == "knowledgehub-observed-native-run/2":
        required = [
            ["cargo", "fmt", "--all", "--", "--check"],
            ["cargo", "test", "--offline", "--all-targets"],
            ["cargo", "test", "--offline", "--doc"],
            ["cargo", "clippy", "--offline", "--all-targets", "--", "-D", "warnings"],
        ]
        if any(command not in [item.get("command") for item in data["commands"]] for command in required):
            raise ValueError("complete required native batch is absent")
        require_checkpoint(root, data["files"])
    else:
        for name, expected in data["files"].items():
            if digest((root / name).read_bytes()) != expected:
                raise ValueError("checked code has changed: " + name)
    for name, expected in data.get("executables", {}).items():
        if digest((root / name).read_bytes()) != expected:
            raise ValueError("checked executable changed: " + name)
    return data
