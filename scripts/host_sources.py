"""Exact host checkpoints shared by native verification, capture and reset preflight."""
import hashlib
import json
import os
import pathlib
import stat
import tomllib

GROUPS = ("source", "design", "delivery", "openspec", "src", "tests", "examples", "scripts")
SUFFIXES = {".rs", ".md", ".pdf", ".toml", ".json", ".txt", ".py", ".yaml", ".yml"}
EXCLUDED = {"evidence", "hub", "old_hub", "orig_hub", "target", "cache", "__pycache__", ".git"}


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
        for base, directories, files in os.walk(root / group, followlinks=False):
            directories[:] = [n for n in directories if n not in EXCLUDED and not (pathlib.Path(base) / n).is_symlink()]
            for name in files:
                path = pathlib.Path(base) / name
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
    if data.get("format") != "knowledgehub-observed-native-run/2":
        raise ValueError("unsupported checked native manifest")
    required = [
        ["cargo", "fmt", "--all", "--", "--check"],
        ["cargo", "test", "--offline", "--all-targets"],
        ["cargo", "test", "--offline", "--doc"],
        ["cargo", "clippy", "--offline", "--all-targets", "--", "-D", "warnings"],
    ]
    if any(command not in [item.get("command") for item in data["commands"]] for command in required):
        raise ValueError("complete required native batch is absent")
    require_checkpoint(root, data["files"])
    for name, expected in data.get("executables", {}).items():
        if digest((root / name).read_bytes()) != expected:
            raise ValueError("checked executable changed: " + name)
    return data


def package_identity(root):
    package = tomllib.loads((root / "Cargo.toml").read_text())["package"]
    entries = tomllib.loads((root / "Cargo.lock").read_text())["package"]
    own = [p for p in entries if p["name"] == "knowledgehub"]
    if package["name"] != "knowledgehub" or package["version"] != "0.10.0" or package.get("publish") is not False or len(own) != 1 or own[0]["version"] != package["version"]:
        raise ValueError("expected unpublished knowledgehub 0.10.0 manifest and lock identity")
    return package["version"]


def retained_inventory(root):
    """Hash host files only, without invoking any superseded store decoder."""
    directory = root / "old_hub"
    if not directory.exists() and not directory.is_symlink():
        return {}
    if directory.is_symlink() or not directory.is_dir():
        raise ValueError("old_hub must be an ordinary retained directory")
    result = {}
    for base, directories, files in os.walk(directory, followlinks=False):
        for name in directories + files:
            path = pathlib.Path(base) / name
            mode = path.lstat().st_mode
            if not (stat.S_ISDIR(mode) or stat.S_ISREG(mode)):
                raise ValueError("retained inventory requires ordinary files/directories")
            if stat.S_ISREG(mode):
                result[str(path.relative_to(directory))] = digest(path.read_bytes())
    return dict(sorted(result.items()))


def load_origins(root):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError("duplicate semantic origin: " + key)
            result[key] = value
        return result
    data = json.loads((root / "examples/self_model_support/origins.json").read_text(), object_pairs_hook=unique)
    for section in ("modules", "public_namespaces", "function_namespaces", "native_contracts"):
        for path in data[section]:
            if not (root / path).is_file():
                raise ValueError("stale semantic origin: " + path)
    for section in ("public_namespaces", "function_namespaces"):
        if set(data[section]) - set(data["modules"]):
            raise ValueError("namespace has no actual module origin")
    proposals = [p for p in source_paths(root) if p.name == "proposal.md" and "changes" in p.parts]
    for path in proposals:
        name = path.parent.name
        if path.parent.parent.name == "archive":
            name = name[11:]
        if name not in data["changes"]:
            raise ValueError("unknown change origin: " + name)
    if set(data["changes"]) != {p.parent.name[11:] if p.parent.parent.name == "archive" else p.parent.name for p in proposals}:
        raise ValueError("stale change origin")
    return data
