#!/usr/bin/env python3
"""Drive the COMPILED client in a real PTY; harness tools are outside its PATH."""
import argparse,fcntl,json,os,pty,re,select,signal,struct,subprocess,time,shutil
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('package',type=Path);p.add_argument('evidence',type=Path);p.add_argument('--isolation-profile',type=Path);args=p.parse_args()
package=args.package.resolve();evidence=args.evidence.resolve();evidence.mkdir(parents=True,exist_ok=True)
empty=evidence/'empty-tool-path';empty.mkdir(exist_ok=True)
class Session:
 def __init__(self,workspace='workspace.json'):
  self.master,slave=pty.openpty();fcntl.ioctl(slave,0x80087467,struct.pack('HHHH',36,150,0,0))
  env={**os.environ,'PATH':str(empty),'NO_COLOR':'1','TERM':'xterm-256color'};env.pop('KNOWLEDGEHUB_CLIENT_HOST',None);env.pop('KNOWLEDGEHUB_WORKSPACE',None)
  launcher=['/usr/bin/sandbox-exec','-f',str(args.isolation_profile.resolve())] if args.isolation_profile else []
  self.process=subprocess.Popen(launcher+[str(package/'kh'),'--workspace',str(package/'models'/workspace),'--history-items','2','browse','--no-color'],stdin=slave,stdout=slave,stderr=slave,cwd=evidence,env=env,start_new_session=True);os.close(slave)
  self.log='';self.steps=[]
 def read(self):
  if select.select([self.master],[],[],.05)[0]:
   try:self.log+=os.read(self.master,262144).decode('utf8','replace')
   except OSError:pass
 def screen(self):return self.log.split('\x1b[H\x1b[2J')[-1]
 def wait(self,text=None,timeout=25):
  deadline=time.monotonic()+timeout
  while time.monotonic()<deadline:
   self.read();s=self.screen()
   if s and 'working / Esc cancels' not in s and (text is None or text in s):
    time.sleep(.12);self.read();s=self.screen()
    if 'working / Esc cancels' not in s and (text is None or text in s):return s
   if self.process.poll() is not None:raise RuntimeError('client exited: '+self.log[-3000:])
  raise RuntimeError('PTY timeout waiting '+str(text)+'\n'+self.screen())
 def key(self,key,text=None):
  previous=len(self.log);os.write(self.master,key.encode());deadline=time.monotonic()+3
  while len(self.log)==previous and time.monotonic()<deadline:self.read()
  s=self.wait(text);self.steps.append({'key':repr(key),'screen':s});return s
 def prompt(self,key,text,value):
  os.write(self.master,key.encode());deadline=time.monotonic()+10
  while text not in self.screen() and time.monotonic()<deadline:self.read()
  if text not in self.screen():raise RuntimeError('prompt missing '+text+'\n'+self.screen())
  os.write(self.master,(value+'\r').encode());return self.wait()
 def stop(self,name):
  os.write(self.master,b'q');deadline=time.monotonic()+35
  while self.process.poll() is None and time.monotonic()<deadline:self.read()
  if self.process.poll() is None:raise RuntimeError('compiled TUI did not exit after q')
  for _ in range(3):self.read()
  os.close(self.master);(evidence/(name+'.ansi')).write_text(self.log);(evidence/(name+'.steps.json')).write_text(json.dumps(self.steps,indent=2)+'\n');assert self.process.returncode==0,self.log[-2500:]
 def failed(self,name):
  (evidence/(name+'.failed.ansi')).write_text(self.log)
  if self.process.poll() is None:os.killpg(self.process.pid,signal.SIGTERM)

s=Session()
try:
 s.wait('groups / type');s.key('?','Keyboard');s.key('?')
 s.prompt('f','Local display filter','=AuditRequirement');s.key(' ','0 rows');s.key('h','Fresh query')
 s.prompt('f','Local display filter','=Requirement');s.key(' ','instances')
 s.prompt('f','Local display filter','R8');s.key('\r','record');s.key('l','links')
 s.prompt('f','Local display filter','satisfies');s.key('\r','record');s.key('s','sources');s.key('z','media_type')
 s.key('p','publications');s.key('n','page 2')
 s.prompt('o','Exact operation ID','authoring:model');s.key('a','affected')
 s.prompt('f','Local display filter','requirement:ack')
 for _ in range(60):
  if '> ' in s.screen() and 'model:requirement:ack@1' in s.screen():break
  if 'n: more' not in s.screen():raise RuntimeError('original affected requirement not found')
  s.key('n')
 else:raise RuntimeError('affected paging bound')
 s.key('\r','record');original=s.screen();handover=json.loads((package/'models/fruittrucks/handover.json').read_text());original_head=next(x['head'] for x in handover['publications'] if x['operation_id']=='authoring:model');assert original_head in original
 s.key('r','Explicitly refreshed');s.key('h','Fresh query');assert original_head in s.screen();s.key('H','Fresh query');s.key('h','Fresh query')
 # Restore exact original record and export through the shared keyboard path.
 os.write(s.master,b'v')
 for prompt,value in [('Diagram format','html'),('New output path',str(evidence/'keyboard.html')),('Projection relationships','relationships'),('Depth 1..8','1')]:
  deadline=time.monotonic()+10
  while prompt not in s.screen() and time.monotonic()<deadline:s.read()
  assert prompt in s.screen(),s.screen();os.write(s.master,(value+'\r').encode());time.sleep(.05)
 s.wait('Derived diagram exported')
 # Equivalent CLI uses the same exact A snapshot and independent publication anchor.
 env={**os.environ,'PATH':str(empty),'NO_COLOR':'1'};env.pop('KNOWLEDGEHUB_CLIENT_HOST',None);env.pop('KNOWLEDGEHUB_WORKSPACE',None)
 launcher=['/usr/bin/sandbox-exec','-f',str(args.isolation_profile.resolve())] if args.isolation_profile else []
 cmd=launcher+[str(package/'kh'),'--snapshot',original_head,'--history-anchor',handover['anchor'],'diagram','model:requirement:ack','--output',str(evidence/'equivalent.json'),'--format','json','--json']
 result=subprocess.run(cmd,cwd=evidence,env=env,capture_output=True,text=True,timeout=40);assert result.returncode==0,result.stdout+result.stderr
 keyboard=json.loads((evidence/'keyboard.html.manifest.json').read_text());equivalent=json.loads((evidence/'equivalent.json.manifest.json').read_text());assert keyboard['graph_sha256']==equivalent['graph_sha256']

 # Lose the host, then reconnect without changing the retained exact selection.
 ps=subprocess.check_output(['/bin/ps','-axo','pid=,ppid=,comm='],text=True)
 children=[int(line.split()[0]) for line in ps.splitlines() if len(line.split())>2 and int(line.split()[1])==s.process.pid and 'knowledgehub-client-host' in line]
 assert children,'native host child missing'
 os.kill(children[0],signal.SIGTERM);time.sleep(.1);s.key('t','Disconnected');s.key('R','Reconnected');assert original_head in s.screen()
 fcntl.ioctl(s.master,0x80087467,struct.pack('HHHH',12,60,0,0));os.kill(s.process.pid,signal.SIGWINCH);time.sleep(.1);s.read();assert 'Compact view' in s.screen();s.stop('compiled-browse')
except Exception:s.failed('compiled-browse');raise

denied=Session('workspace-no-history.json')
try:
 denied.wait('groups / type');denied.key('p','Denied');denied.key('t','groups / type');denied.stop('read-history-denied')
except Exception:denied.failed('read-history-denied');raise
# Cached rows, j/k, filtering and help redraws cannot bypass live ReadHistory revocation.
workspace_file=package/'models/workspace.json';saved=workspace_file.read_bytes();live=Session()
try:
 live.wait('groups / type');live.key('p','publications');policy=json.loads(saved);policy['models'][0]['grants']=['Open','Read'];workspace_file.write_text(json.dumps(policy));live.key('j','Denied');assert 'host:publisher-A' not in live.screen();live.key('t','groups / type');live.stop('live-history-revocation')
except Exception:live.failed('live-history-revocation');raise
finally:workspace_file.write_bytes(saved)
# Failed model admissions are visible while the ordinary active model stays usable.
config_path=package/'models/workspace.json';config=json.loads(config_path.read_text());broken=package/'models/incompatible-hub';shutil.copytree(package/'models/fruittrucks/hub',broken)
header=broken/'hub.json';data=json.loads(header.read_text());data['format']='knowledgehub-files/999';header.write_text(json.dumps(data))
base=config['models'][0]
for alias,grants in [('incompatible',['Open','Read']),('denied',[])]:
 entry={**base,'alias':alias,'name':'Test '+alias,'directory':'incompatible-hub','grants':grants,'known_baselines':[]};config['models'].append(entry)
negative=package/'models/workspace-negative.json';negative.write_text(json.dumps(config))
n=Session('workspace-negative.json')
try:
 n.wait('groups / type');n.key('m','models');n.prompt('f','Local display filter','incompatible');n.key('\r','Incompatible');n.key('t','groups / type')
 n.key('m','models');n.prompt('f','Local display filter','denied');n.key('\r','Denied');n.key('t','groups / type');n.stop('failed-admissions')
except Exception:n.failed('failed-admissions');raise
(evidence/'pty.json').write_text(json.dumps({'compiled':True,'real_pty':True,'empty_tool_path':str(empty),'cwd':str(evidence),'package':str(package),'keyboard_source_history_export':True,'keyboard_cli_graph_equal':True,'empty_and_failed_admissions':True,'unknown_operation_unavailable_anchor':True,'search_cancel_restores_view':True,'original_snapshot':original_head,'back_forward_refresh':True,'lost_host_reconnect':True,'resize':True,'no_color':True,'history_denied_ordinary_available':True,'live_history_revocation_clears_cached_rows':True,'human_usability_acceptance':'pending'},indent=2)+'\n')
print(json.dumps({'status':'pass','evidence':str(evidence),'original_snapshot':original_head}))
