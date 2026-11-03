#!/usr/bin/env python3
"""Observe one installed agent host against an isolated, maintained fixture replay.

This does not claim independent plan authoring. It never changes host settings.
"""
import argparse
import hashlib
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
p = argparse.ArgumentParser()
p.add_argument('host', choices=['codex', 'claude', 'copilot'])
p.add_argument('--run-id', required=True)
args = p.parse_args()
work = Path('/private/tmp') / ('knowledgehub-host-' + args.run_id + '-' + args.host)
work.mkdir(exist_ok=False)
skill_root = {'codex': '.agents', 'claude': '.claude', 'copilot': '.github'}[args.host]
for name in ['knowledgehub-import', 'knowledgehub-verify']:
    shutil.copytree(ROOT / 'skills' / name, work / skill_root / 'skills' / name)
fixture = ROOT / 'examples/fixtures/unrelated-design'
shutil.copytree(fixture / 'source', work / 'source')
for name in ['model-plan.json', 'coverage.json']:
    shutil.copy2(fixture / name, work / name)
shutil.copy2(ROOT / 'target/debug/knowledgehub-model', work / 'knowledgehub-model')
manifest = json.loads((fixture / 'source-manifest.json').read_text())
grants = ['Create', 'Open', 'Read', 'Validate', 'Commit', 'RegisterOntology', 'OperationResult']
(work / 'host.json').write_text(json.dumps(dict(actor='host:analyst', grants=grants)))
config = dict(format=manifest['format'], source=str(work / 'source'), destination=str(work / 'hub'), work=str(work / 'run'), actor='host:analyst', grants=grants, scope=manifest['scope'], inputs=manifest['inputs'], authority=manifest['authority'])
(work / 'config.json').write_text(json.dumps(config))
version = subprocess.run([args.host, '--version'], capture_output=True, text=True)
result = dict(host=args.host, version=version.stdout.strip(), workspace=str(work), mode='reviewed maintained-plan replay; not newly authored', binary_sha256=hashlib.sha256((work/'knowledgehub-model').read_bytes()).hexdigest(), stages=[])
for stage in ['import', 'verify']:
    if stage == 'import':
        task = 'Use the knowledgehub-import skill discovered in this project. Read its instructions and inspect source/design.md, source/rule.js and the supplied model-plan.json/coverage.json as data. This is a maintained plan replay, not new authoring. Execute ./knowledgehub-model catalogue catalogue.json, capture config.json, prepare run model-plan.json coverage.json, assess run host.json, create run host.json, resolve run host.json. Retain all explicit gaps and report actual status. Do not modify sources, plans, skills, settings or backend files. Do not run rule.js. Do not delegate or spawn agents. Only this workspace is writable. Stop with the actual prerequisite if execution is unavailable.'
    else:
        task = 'Use the knowledgehub-verify skill discovered in this project. This is a fresh independent verification process of the retained native result. Read the skill, then execute ./knowledgehub-model resolve run host.json and ./knowledgehub-model verify run host.json. Inspect run/verification.json and return actual validity, completeness, coverage, baseline and pending human meaning review. Do not edit the source, plan, settings, skills or backend files. Do not delegate or spawn agents.'
    if args.host == 'codex':
        command = ['codex', 'exec', '--ephemeral', '--skip-git-repo-check', '--sandbox', 'workspace-write', '--json', '-C', str(work), task]
    elif args.host == 'claude':
        command = ['claude', '--print', '--output-format', 'json', '--max-budget-usd', '3', '--allowedTools', 'Read,Bash(./knowledgehub-model *)', '--', task]
    else:
        command = ['copilot', '--prompt', task, '--trust-folder', '--no-auto-update', '--no-ask-user', '--disable-builtin-mcps', '--allow-tool=shell(./knowledgehub-model:*)', '--max-ai-credits', '30', '--output-format', 'json']
    (work / (stage + '-command.json')).write_text(json.dumps(command, indent=2))
    try:
        completed = subprocess.run(command, cwd=work, capture_output=True, text=True, timeout=300)
        code, stdout, stderr = completed.returncode, completed.stdout, completed.stderr
    except subprocess.TimeoutExpired as exc:
        code, stdout, stderr = 124, exc.stdout or b'', exc.stderr or b''
        stdout = stdout.decode() if isinstance(stdout, bytes) else stdout
        stderr = stderr.decode() if isinstance(stderr, bytes) else stderr
    (work / (stage + '.stdout.log')).write_text(stdout)
    (work / (stage + '.stderr.log')).write_text(stderr)
    result['stages'].append(dict(stage=stage, returncode=code, verification_exists=(work/'run/verification.json').is_file()))
    (work / 'host-result.json').write_text(json.dumps(result, indent=2)+'\n')
    if code or not (work/'run/verification.json').is_file():
        break
result['passed'] = len(result['stages']) == 2 and all(s['returncode'] == 0 and s['verification_exists'] for s in result['stages'])
print(json.dumps(result))
