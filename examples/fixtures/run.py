#!/usr/bin/env python3
"""Replay a maintained neutral fixture into a new native destination.

Never resets a destination. All generated stores/journals remain under target.
"""
import argparse, hashlib, json, shutil, subprocess
from pathlib import Path
from datetime import datetime, timezone
ROOT=Path(__file__).resolve().parents[2]
parser=argparse.ArgumentParser();parser.add_argument('fixture',choices=['memento','unrelated-design']);parser.add_argument('--run-id',default=datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ'));parser.add_argument('--variant',choices=['missing-requirement','missing-ui']);parser.add_argument('--skip-assess',action='store_true');parser.add_argument('--binary',type=Path,default=ROOT/'target/debug/knowledgehub-model');args=parser.parse_args()
fixture=Path(__file__).resolve().parent/args.fixture
output=ROOT/'target/rich-design'/args.run_id;output.mkdir(parents=True,exist_ok=False)
manifest=json.loads((fixture/'source-manifest.json').read_text())
grants=['Create','Open','Read','Validate','Commit','RegisterOntology','OperationResult']
host=output/'host.json';host.write_text(json.dumps({'actor':'host:analyst','grants':grants},indent=2)+'\n')
config=dict(format=manifest['format'],source=str(fixture/'source'),destination=str(output/'hub'),work=str(output/'run'),actor='host:analyst',grants=grants,scope=manifest['scope'],inputs=manifest['inputs'],authority=manifest['authority'])
(output/'config.json').write_text(json.dumps(config,indent=2)+'\n')
binary=output/'knowledgehub-model'
shutil.copy2(args.binary,binary)
(output/'executable.json').write_text(json.dumps({'sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'length':binary.stat().st_size},indent=2)+'\n')
plan=fixture/'model-plan.json'
if args.variant:
    if args.fixture!='memento':raise SystemExit('negative variants require the Memento reference')
    value=json.loads(plan.read_text());old='R-32' if args.variant=='missing-requirement' else 'interaction:execute';new='deliberately-replaced:'+old
    def rename(v):
        if isinstance(v,dict):
            if v.get('key')==old:v['key']=new
            if v.get('$ref')==old:v['$ref']=new
            for x in v.values():rename(x)
        elif isinstance(v,list):
            for x in v:rename(x)
    rename(value);plan=output/'variant-plan.json';plan.write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n')
    (output/'variant.json').write_text(json.dumps({'variant':args.variant,'removed_logical_id':old,'replacement':new,'record_count_unchanged':True,'coverage_expectations_unchanged':True},indent=2)+'\n')
commands=[['catalogue',str(output/'catalogue.json')],['capture',str(output/'config.json')],['prepare',str(output/'run'),str(plan),str(fixture/'coverage.json')]]
if not args.skip_assess:commands.append(['assess',str(output/'run'),str(host)])
commands.extend([['create',str(output/'run'),str(host)],['resolve',str(output/'run'),str(host)],['verify',str(output/'run'),str(host)]])
for index,command in enumerate(commands):
    completed=subprocess.run([str(binary),*command],cwd=ROOT,capture_output=True,text=True)
    (output/f'{index}-{command[0]}.stdout.json').write_text(completed.stdout)
    (output/f'{index}-{command[0]}.stderr.log').write_text(completed.stderr)
    if completed.returncode:
        print(json.dumps({'output':str(output),'failed_command':command,'returncode':completed.returncode,'diagnostic':completed.stderr[:8000]}));raise SystemExit(completed.returncode)
print(json.dumps({'output':str(output),'verification':str(output/'run/verification.json'),'status':json.loads((output/'run/verification.json').read_text())['status']}))
