#!/usr/bin/env python3
"""Execute native checks and bind their original output to the exact checked working tree."""
import datetime,hashlib,json,pathlib,subprocess,sys
root=pathlib.Path(__file__).resolve().parents[1]
def digest(b):return dict(sha256=hashlib.sha256(b).hexdigest(),length=len(b))
paths=sorted([*root.glob('Cargo.*'),root/'README.md',*root.glob('src/**/*.rs'),*root.glob('src/**/*.json'),*root.glob('source/*'),*[p for p in root.glob('tests/fixtures/files1/**/*') if p.is_file()],*root.glob('tests/**/*.rs'),*root.glob('examples/**/*.rs'),root/'examples/capture_self_model.py',pathlib.Path(__file__)])
files={str(p.relative_to(root)):digest(p.read_bytes()) for p in paths}
start=datetime.datetime.now(datetime.timezone.utc).isoformat();out=root/'delivery/evidence/full-ontology-run.txt';manifest=root/'delivery/evidence/full-ontology-checked.json'
commands=[['cargo','fmt','--check'],['cargo','test','--offline','--','--nocapture'],['cargo','clippy','--offline','--all-targets','--','-D','warnings']]
results=[]
with out.open('wb') as log:
 for command in [['rustc','-Vv'],['findmnt','-T',str(root),'-J','-o','TARGET,SOURCE,FSTYPE,OPTIONS'],*commands]:
  log.write(('\n$ '+' '.join(command)+'\n').encode());log.flush();print('$ '+' '.join(command),flush=True)
  result=subprocess.run(command,cwd=root,stdout=log,stderr=subprocess.STDOUT)
  results.append(dict(command=command,exit_code=result.returncode))
  if result.returncode:
   print(f'Failed; original output: {out}',flush=True);sys.exit(result.returncode)
 assert all(digest((root/p).read_bytes())==h for p,h in files.items()), 'source changed while checks ran'
metadata=subprocess.run(['cargo','test','--offline','--no-run','--message-format=json'],cwd=root,capture_output=True,text=True,check=True)
executables={}
for line in metadata.stdout.splitlines():
 try:item=json.loads(line)
 except json.JSONDecodeError:continue
 if item.get('executable'):
  executable=pathlib.Path(item['executable']);executables[str(executable.relative_to(root))]=digest(executable.read_bytes())
manifest.write_text(json.dumps(dict(format='knowledgehub-observed-native-run/1',started_at=start,ended_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_status='exact uncommitted working tree; no Git commit identity substituted',files=files,executables=executables,build_metadata_command=['cargo','test','--offline','--no-run','--message-format=json'],commands=results,log=digest(out.read_bytes()),limits=['Linux/ext4 native tests and process interruption; no power-loss evidence','No human acceptance, remote adapter parity or authority transfer']),indent=2)+'\n')
print(f'Native checks passed; checked-source evidence: {manifest}',flush=True)
