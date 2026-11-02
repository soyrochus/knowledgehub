import sys
import pathlib
import json
import os
import subprocess
import time
import datetime
sys.dont_write_bytecode = True
ROOT = pathlib.Path('/Users/ivanderk/src/knowledgehub')
sys.path.insert(0, str(ROOT / 'scripts'))
from host_sources import checked_run, digest, require_checkpoint
from rebuild_canonical_hub import command, packed_cache_report, verify_reopened

directory = ROOT / 'delivery/evidence/macos-file-backend/macos-20261102-01'
checked = checked_run(ROOT, directory / 'checked.json', directory / 'native.log')
fixtures = ROOT / 'target/macos-file-backend-20261102-01'
fixtures.mkdir(exist_ok=False)
results = []
binaries = ROOT / 'target/release/examples'
def run(label, args):
    print(label, flush=True)
    return command(ROOT, directory, label, args, results)

for example in ('inspect_self_model', 'full_self_description', 'catalogue_conformance'):
    label = 'obsolete-format-' + example
    args = [str(binaries / example), '--format', 'files3']
    start = time.monotonic()
    rejected = subprocess.run(args, cwd=ROOT, capture_output=True)
    assert rejected.returncode != 0 and b'unknown host argument --format' in rejected.stderr
    (directory / (label + '.stderr.log')).write_bytes(rejected.stderr)
    results.append(dict(name=label, command=args, exit_code=rejected.returncode, expected='obsolete argument refused',
                        seconds=time.monotonic()-start, stderr=digest(rejected.stderr)))
run('minimal', [binaries / 'minimal_model', fixtures / 'minimal'])
catalogue = run('catalogue', [binaries / 'catalogue_conformance', fixtures / 'catalogue'])
run('shared-storage', [binaries / 'shared_storage', 'create', fixtures / 'shared'])
run('neighbours', [binaries / 'neighbours', fixtures / 'neighbours'])
run('rent-impact', [binaries / 'rent_impact', fixtures / 'rent-impact'])
run('storage-benchmark', [binaries / 'storage_benchmark', fixtures / 'storage-benchmark', 'small'])
run('neighbour-benchmark', [binaries / 'neighbour_benchmark', fixtures / 'neighbour-benchmark'])
for example in ('storage-benchmark', 'neighbour-benchmark'):
    value = json.loads((directory / (example + '.json')).read_text())
    metric = value['peak_process_rss_bytes']
    assert metric is None or metric > 0
    print(example, 'peak RSS bytes', metric, flush=True)
relative = directory.relative_to(ROOT)
run('capture', [sys.executable, ROOT / 'examples/capture_self_model.py', '--output', relative / 'self-model-input.json',
                '--derived', relative / 'ontology-pdf-extracted.txt', '--log', relative / 'native.log',
                '--checked-manifest', relative / 'checked.json'])
run('validate-input', [binaries / 'full_self_description', '--validate-input', directory / 'self-model-input.json'])
run('author', [binaries / 'full_self_description', directory / 'self-model-input.json', fixtures / 'self-model', '--result', directory / 'self-model-result.json'])
pre = run('inspect-before-compact', [binaries / 'inspect_self_model', '--verify', fixtures / 'self-model'])
first = run('inspect-compacted', [binaries / 'inspect_self_model', '--verify', '--compact', fixtures / 'self-model'])
second = run('inspect-reopened', [binaries / 'inspect_self_model', '--verify', fixtures / 'self-model'])
before, compacted, reopened = [json.loads(p.read_text()) for p in (pre, first, second)]
prepost = verify_reopened(before, compacted)
continuity = verify_reopened(compacted, reopened)
caches = packed_cache_report(fixtures / 'self-model', reopened)
run('coverage', [sys.executable, ROOT / 'scripts/audit_full_ontology.py', catalogue, directory / 'self-model-result.json', directory / 'self-model-input.json',
                 '--log', relative / 'native.log', '--output', relative / 'coverage-audit.json'])
require_checkpoint(ROOT, checked['files'])

def inventory(path):
    result = {}
    for base, dirs, files in os.walk(path, followlinks=False):
        for name in files:
            p = pathlib.Path(base) / name
            if p.is_symlink(): raise ValueError('unexpected retained symlink')
            result[str(p.relative_to(path))] = digest(p.read_bytes())
    return dict(sorted(result.items()))
retained = dict(old_hub=inventory(ROOT / 'old_hub'), hub=inventory(ROOT / 'hub'), historical_evidence={k:v for k,v in inventory(ROOT / 'delivery/evidence').items() if not k.startswith('macos-file-backend/')})
assert retained == json.loads((directory.parent / 'feasibility/retained-before.json').read_text())
(directory / 'retained-after.json').write_text(json.dumps(retained, indent=2) + '\n')
report = dict(format='knowledgehub-macos-isolated-walkthrough/1', observed_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
              platform=checked['platform'], checked_source=checked['files'], checked_log=checked['log'], commands=results,
              pre_post_compaction=prepost, reopened=continuity, caches=caches, fixtures=str(fixtures),
              retained_unchanged=True, old_hub_present=(ROOT / 'old_hub').exists(),
              head=reopened['head'], selected_records=reopened['selected_records'], retained_revisions=reopened['retained_revisions'],
              explicit_gaps=reopened['explicit_gaps'], scrub=reopened['scrub'],
              linux='assumed at explicit user direction; native run deferred to separate machine',
              cross_host='both native copy directions unverified',
              limits=['process faults/injected sync failures only', 'no power-loss/kernel-crash/hardware claim', 'no human acceptance/authority handover/publication/canonical refresh'])
(directory / 'walkthrough.json').write_text(json.dumps(report, indent=2) + '\n')
print('All isolated Mac walkthroughs and retained-byte comparisons passed', flush=True)
