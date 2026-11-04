#!/usr/bin/env python3
"""Prepare two NEW disposable native Hubs; never resets or mutates an input Hub."""
import argparse,hashlib,json,shutil,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FIX=ROOT/'examples/fixtures/terminal-client'
def write(path,value):path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n')
def main():
    parser=argparse.ArgumentParser();parser.add_argument('destination',type=Path);parser.add_argument('--release',action='store_true');args=parser.parse_args()
    dest=args.destination.absolute()
    if dest.exists():raise SystemExit('occupied destination refused; choose a fresh directory')
    if any(dest==ROOT/x or ROOT/x in dest.parents for x in ['hub','old_hub']):raise SystemExit('protected store')
    dest.mkdir(parents=True)
    profile='release' if args.release else 'debug';binary=ROOT/'target'/profile/'knowledgehub-model'
    def run(command,log):
        p=subprocess.run([str(x) for x in command],cwd=ROOT,capture_output=True,text=True)
        log.with_suffix('.stdout').write_text(p.stdout);log.with_suffix('.stderr').write_text(p.stderr)
        if p.returncode:raise SystemExit(f'{command[0]} {command[1]} failed: {p.stderr[:4000]}')
    entries=[]
    for name,logical in [('fruittrucks','requirement:ack'),('equipment','entity:Booking')]:
        f=FIX/name;out=dest/name;out.mkdir()
        manifest=json.loads((f/'source-manifest.json').read_text());grants=['Create','Open','Read','ReadHistory','Validate','Commit','RegisterOntology','OperationResult']
        host=out/'publisher.json';write(host,dict(actor='host:publisher-A',grants=grants))
        write(out/'config.json',dict(format=manifest['format'],source=str(f/'source'),destination=str(out/'hub'),work=str(out/'run'),actor='host:publisher-A',grants=grants,scope=manifest['scope'],inputs=manifest['inputs'],authority=manifest['authority']))
        for command in [['capture',out/'config.json'],['prepare',out/'run',f/'model-plan.json',f/'coverage.json'],['assess',out/'run',host],['create',out/'run',host],['resolve',out/'run',host],['verify',out/'run',host]]:run([binary,*command],out/command[0])
        generated=out/'run/generation-manifest.json'
        run([ROOT/'target'/profile/'examples/model_successor',generated,logical,host,out/'successor.json'],out/'successor')
        run([ROOT/'target'/profile/'examples/terminal_fixture_handover',generated,host,out/'handover.json'],out/'handover')
        hand=json.loads((out/'handover.json').read_text())
        shutil.copytree(f,out/'reference')
        entries.append(dict(alias=name,name='FruitTrucks route-change design' if name=='fruittrucks' else 'Equipment reservation design',directory=f'{name}/hub',scope=manifest['scope'],actor='host:reader-B',grants=['Open','Read','ReadHistory'],known_baselines=hand['baselines']))
    write(dest/'workspace.json',dict(format='knowledgehub-client-workspace/1',default='fruittrucks',models=entries))
    for entry in entries:entry['grants']=['Open','Read']
    write(dest/'workspace-no-history.json',dict(format='knowledgehub-client-workspace/1',default='fruittrucks',models=entries))
    print(json.dumps(dict(destination=str(dest),workspace=str(dest/'workspace.json'))))
if __name__=='__main__':main()
