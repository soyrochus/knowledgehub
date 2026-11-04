#!/usr/bin/env python3
"""Make source checkout/development tools/network unavailable to the end-user pair."""
import argparse,json,os,shutil,subprocess,tarfile
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('archive',type=Path);p.add_argument('destination',type=Path);args=p.parse_args()
root=Path(__file__).resolve().parents[3];out=args.destination.resolve();out.mkdir(parents=True,exist_ok=False)
with tarfile.open(args.archive) as tar:
 for m in tar.getmembers():
  if m.issym() or m.islnk() or Path(m.name).is_absolute() or '..' in Path(m.name).parts or any(x.startswith('._') for x in Path(m.name).parts):raise RuntimeError('unsafe/nonordinary archive')
 tar.extractall(out)
package=next(x for x in out.iterdir() if x.is_dir());renamed=out/'package with spaces';package.rename(renamed);package=renamed
profile=out/'isolation.sb';profile.write_text('(version 1)\n(allow default)\n(deny network*)\n(deny file-read* (subpath '+json.dumps(str(root.parent))+'))\n(deny process-exec (regex #"/(bun|node|cargo|rustc)$"))\n')
prefix=['/usr/bin/sandbox-exec','-f',str(profile)]
empty=out/'empty-tool-path';empty.mkdir();env={**os.environ,'PATH':str(empty),'NO_COLOR':'1'}
for k in ['KNOWLEDGEHUB_CLIENT_HOST','KNOWLEDGEHUB_WORKSPACE']:env.pop(k,None)
cwd=out/'another cwd';cwd.mkdir();results=[]
def run(command,expected=0):
 r=subprocess.run(prefix+command,cwd=cwd,env=env,capture_output=True,text=True,timeout=45)
 if r.returncode!=expected:raise RuntimeError(f'{command} exit {r.returncode}, expected {expected}\n{r.stdout[:3000]}\n{r.stderr[:2000]}')
 results.append(dict(command=command,exit=r.returncode,stdout=r.stdout,stderr=r.stderr));return r
probe=run(['/bin/cat',str(root/'Cargo.toml')],1);assert 'not permitted' in probe.stderr.lower() or 'permission denied' in probe.stderr.lower()
blocked=[]
for name in ['bun','node','cargo','rustc']:
 path=shutil.which(name)
 if path:
  r=subprocess.run(prefix+[path,'--version'],cwd=cwd,env=env,capture_output=True,text=True);assert r.returncode!=0,(name,r.stdout);blocked.append(dict(tool=name,path=path,exit=r.returncode))
kh=str(package/'kh');run([kh,'--help']);info=json.loads(run([kh,'info','--json']).stdout);assert info['model']['reader']=='host:reader-B'
run([kh,'publications','--history-items','1','--json']);run([kh,'publication','authoring:model','--affected','--json']);run([kh,'show','model:requirement:ack','--json']);run([kh,'--model','equipment','type','SpecialDefinition','--json']);run([kh,'--workspace',str(package/'models/workspace-no-history.json'),'publications','--json'],4)
run([kh,'diagram','model:requirement:ack','--output',str(cwd/'isolated.html'),'--format','html','--json'])
(out/'isolation.json').write_text(json.dumps(dict(status='pass',package=str(package),profile=str(profile),source_read_blocked=True,development_tools_blocked=blocked,network='denied by OS sandbox',commands=results),indent=2)+'\n')
print(json.dumps(dict(status='pass',package=str(package),profile=str(profile),evidence=str(out))))
