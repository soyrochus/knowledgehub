#!/usr/bin/env python3
"""Extract and exercise compiled distribution with an empty development-tool PATH."""
import argparse,hashlib,json,os,subprocess,tarfile
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('archive',type=Path);p.add_argument('evidence',type=Path);args=p.parse_args()
out=args.evidence.resolve();out.mkdir(parents=True,exist_ok=False);extracted=out/'extracted';extracted.mkdir()
with tarfile.open(args.archive) as tar:
 for member in tar.getmembers():
  if member.issym() or member.islnk() or Path(member.name).is_absolute() or '..' in Path(member.name).parts:raise RuntimeError('unsafe archive member')
 tar.extractall(extracted)
original=next(extracted.iterdir());package=extracted/'package with spaces';original.rename(package)
empty=out/'empty-tool-path';empty.mkdir();cwd=out/'another cwd';cwd.mkdir();env={**os.environ,'PATH':str(empty),'NO_COLOR':'1'}
for key in ['KNOWLEDGEHUB_CLIENT_HOST','KNOWLEDGEHUB_WORKSPACE']:env.pop(key,None)
manifest=json.loads((package/'checksums.json').read_text())
for name,sha in manifest['files'].items():assert hashlib.sha256((package/name).read_bytes()).hexdigest()==sha,name
def semantic():
 return {str(p.relative_to(package)):hashlib.sha256(p.read_bytes()).hexdigest() for n in ['fruittrucks','equipment'] for stem in ['hub.json','objects','refs'] for p in ([package/'models'/n/'hub'/stem] if stem=='hub.json' else (package/'models'/n/'hub'/stem).rglob('*')) if p.is_file()}
before=semantic();results=[]
def run(arguments,expected=0,raw=False):
 r=subprocess.run([str(package/'kh'),*arguments],cwd=cwd,env=env,capture_output=True,text=True,timeout=40)
 if r.returncode!=expected:raise RuntimeError(f'{arguments}: exit {r.returncode}, expected {expected}\n{r.stdout[:4000]}\n{r.stderr[:1000]}')
 if raw:results.append(dict(arguments=arguments,exit=r.returncode));return r.stdout
 value=json.loads(r.stdout);assert value['format']=='knowledgehub-exploration/1'
 if 'coverage' in value and 'encoded_output_bytes' in value['coverage']:assert int(value['coverage']['encoded_output_bytes'])==len(r.stdout.encode()),arguments
 results.append(dict(arguments=arguments,exit=r.returncode,result=value));return value
run(['--help'],raw=True);run(['--version'],raw=True);run(['models','--json'])
for model in ['fruittrucks','equipment']:
 context=['--model',model];hand=json.loads((package/'models'/model/'handover.json').read_text());info=run(context+['info','--json']);assert info['data']['head']==hand['anchor'];assert info['data']['reader']=='host:reader-B'
 pubs=run(context+['publications','--history-items','1','--json']);assert pubs['status']=='complete';assert [x['operation_id'] for x in pubs['data']['items']]==[x['operation_id'] for x in hand['publications']]
 for receipt in hand['publications']:
  got=run(context+['publication',receipt['operation_id'],'--affected','--history-items','37','--json'])['data'];h=got['publication']['outcome']['Published'];assert h['actor']==receipt['actor']=='host:publisher-A';assert h['fingerprint']==receipt['fingerprint'];assert h['previous_head']==receipt['previous_head'];assert h['head']==receipt['head'];assert got['affected']['items']==receipt['affected']
 selected=run(context+['baselines','--json'])['data'];assert len(selected['baselines'])==3;assert not selected['retained']['complete']
 retained=run(context+['baselines','--retained','--json'])['data'];assert retained['retained']['complete'];assert len(retained['baselines'])==3
 a,b=map(json.dumps,hand['baselines'][:2]);run(context+['--baseline',a,'info','--json']);diff=run(context+['compare',a,b,'--json']);assert diff['data']['differences']
 for i,source in enumerate(hand['sources']):
  path=cwd/f'{model}-source-{i}.bin';run(context+['source',json.dumps(source['reference']),'--export',str(path),'--json']);assert hashlib.sha256(path.read_bytes()).hexdigest()==source['original']['sha256']
 limited=run(context+['publications','--items','1','--json'],3);assert limited['status']=='partial';assert len(limited['data']['items'])==1
 zero=run(context+['publications','--history-nodes','1','--json'],3);assert zero['data']['stopped_error']['code']=='HistoryLimit'
 missing=run(context+['--history-anchor','a'*64,'publication','absent','--json'],6);assert missing['error']['code']=='HistoryAnchorUnavailable'
 earliest=hand['publications'][-1]['head'];absent=run(context+['--history-anchor',earliest,'publication',hand['publications'][0]['operation_id'],'--json']);assert absent['data']['outcome']=='NotPublishedAt'
run(['--workspace',str(package/'models/workspace-no-history.json'),'types','--json'])
deny=run(['--workspace',str(package/'models/workspace-no-history.json'),'publications','--json'],4);assert deny['error']['action']=='ReadHistory'
run(['show','nonexistent:selected-id','--json'],5);run(['publications','--history-items','18446744073709551616','--json'],2);run(['search','ac','--type','Requirement','--json'],2);run(['browse','--json'],2)
run(['show','Open acknowledgement policy','--json'],5)
run(['--model','equipment','type','SpecialDefinition','--json'])
run(['source','model:capture:design.md','--preview','--json'])
for projection,subject in [('relationships','model:requirement:ack'),('lifecycle','model:lifecycle:offer'),('journey','model:journey:change')]:
 for fmt,suffix in [('json','json'),('mermaid','mmd'),('html','html')]:run(['diagram',subject,'--projection',projection,'--output',str(cwd/f'{projection}.{suffix}'),'--format',fmt,'--json'])
run(['diagram','model:requirement:ack','--output',str(cwd/'limited.json'),'--format','json','--vertices','1','--json'],3)
run(['diagram','model:requirement:ack','--output',str(cwd/'relationships.html'),'--json'],6)
after=semantic();assert before==after,'semantic store files changed during exploration'
(out/'commands.json').write_text(json.dumps(results,indent=2)+'\n');(out/'package.json').write_text(json.dumps(dict(status='pass',package=str(package),cwd=str(cwd),tool_path=str(empty),commands=len(results),checksums=len(manifest['files']),semantic_files=len(before),semantic_before_after_equal=True,targets=dict(macOS='arm64/APFS',linux='not verified',signing='not signed/not notarized',publication='not published'),human_acceptance='pending'),indent=2)+'\n')
print(json.dumps(dict(status='pass',package=str(package),commands=len(results),evidence=str(out))))
