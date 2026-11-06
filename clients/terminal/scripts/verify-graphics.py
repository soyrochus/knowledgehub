#!/usr/bin/env python3
"""Compiled-client PTY protocol simulation. This is NOT native pixel-display evidence."""
import argparse, base64, fcntl, hashlib, json, os, pty, re, select, signal, struct, subprocess, time
from pathlib import Path

p = argparse.ArgumentParser()
p.add_argument('package', type=Path)
p.add_argument('evidence', type=Path)
p.add_argument('--isolation-profile', type=Path)
p.add_argument('--asset-failure-client', type=Path)
p.add_argument('--expect-placement-failure', action='store_true', help='reproduce the superseded clear-after-upload bug')
args = p.parse_args()
package, out = args.package.resolve(), args.evidence.resolve()
out.mkdir(parents=True, exist_ok=False)
empty = out / 'empty-tool-path'; empty.mkdir()
env = {**os.environ, 'PATH': str(empty), 'TERM': 'xterm-256color', 'NO_COLOR': '1'}
for key in ['KNOWLEDGEHUB_CLIENT_HOST', 'KNOWLEDGEHUB_WORKSPACE', 'TMUX', 'STY']:
    env.pop(key, None)
prefix = ['/usr/bin/sandbox-exec', '-f', str(args.isolation_profile.resolve())] if args.isolation_profile else []
results = []

def semantic():
    files = []
    for name in ['fruittrucks', 'equipment']:
        model = package / 'models' / name
        files += [model/'handover.json', model/'successor.json', model/'hub/hub.json']
        for folder in ['reference', 'hub/objects', 'hub/refs']:
            files += [p for p in (model/folder).rglob('*') if p.is_file()]
    return {str(p.relative_to(package)): hashlib.sha256(p.read_bytes()).hexdigest() for p in files}

before = semantic()
class Session:
    def __init__(self, command, mode='ok', workspace='workspace-no-history.json'):
        self.master, slave = pty.openpty()
        self.rows, self.cols = 36, 150
        fcntl.ioctl(slave, 0x80087467, struct.pack('HHHH', self.rows, self.cols, 1440, 900))
        self.proc = subprocess.Popen(prefix + [str(args.asset_failure_client.resolve() if args.asset_failure_client else package/'kh'), '--workspace', str(package/'models'/workspace), *command], stdin=slave, stdout=slave, stderr=slave, cwd=out, env=env, start_new_session=True)
        os.close(slave)
        self.mode, self.raw, self.buffer = mode, b'', b''
        self.images, self.placements = set(), set()
        self.upload_id, self.encoded, self.frames, self.probes = None, b'', [], 0
        self.foreign_image_preserved = True
        self.erased_uploaded_images = 0
        self.placement_failures = []
    def send(self, data): os.write(self.master, data.encode() if isinstance(data,str) else data)
    def read(self):
        if not select.select([self.master], [], [], .02)[0]: return
        try: chunk = os.read(self.master, 1024*1024)
        except OSError: return
        self.raw += chunk; self.buffer += chunk
        pattern = rb'\x1b_G([^;]*);([^\x1b]*)\x1b\\|\x1b\[(14|18)t|(\x1b\[[23]J)'
        consumed = 0
        for match in re.finditer(pattern, self.buffer):
            consumed = match.end()
            if match[4]:
                self.erased_uploaded_images += len(self.images)
                self.images.clear();self.placements.clear()
                self.foreign_image_preserved=False
                continue
            if match[3]:
                if self.mode != 'geometry-missing':
                    self.send(f'\x1b[4;900;1440t' if match[3] == b'14' else f'\x1b[8;{self.rows};{self.cols}t')
                continue
            c = dict(x.split('=',1) for x in match[1].decode().split(',') if '=' in x)
            action = c.get('a')
            if action == 'q':
                self.probes += 1
                if self.mode == 'device-only': self.send('\x1b[?62;4c')
                elif self.mode == 'truncated':self.send(f"\x1b_Gi={c['i']};O")
                elif self.mode != 'no-response': self.send(f"\x1b_Gi={c['i']};{'ENOTSUP' if self.mode=='unsupported' else 'OK'}\x1b\\")
            elif action == 't': self.upload_id=int(c['i']); self.images.add(self.upload_id); self.encoded=b''
            elif action == 'p':
                image=int(c['i'])
                if image in self.images:
                    self.placements.add(image);payload='OK'
                else:
                    payload='ENOENT: image not found';self.placement_failures.append(dict(image_id=image,reply=payload))
                placement=f",p={c['p']}" if 'p' in c else ''
                self.send(f"\x1b_Gi={image}{placement};{payload}\x1b\\")
            elif action == 'd':
                assert c.get('d') in ['i','I'], c
                image=int(c['i']);self.placements.discard(image)
                if c['d']=='I':self.images.discard(image)
            if 'm' in c:
                assert len(match[2]) <= 4096
                self.encoded += match[2]
                if c['m']=='0':
                    png=base64.b64decode(self.encoded,validate=True)
                    assert png[:8]==b'\x89PNG\r\n\x1a\n'
                    assert len(png)<=8*1024*1024
                    self.frames.append(png)
                    if self.mode != 'ack-missing': self.send(f"\x1b_Gi={self.upload_id};{'EIO' if self.mode=='ack-negative' else 'OK'}\x1b\\")
        self.buffer=self.buffer[consumed:]
        if len(self.buffer)>65536 and b'\x1b_G' not in self.buffer:self.buffer=self.buffer[-16:]
    def screen(self):
        text=re.sub(rb'\x1b_G[^\x1b]*\x1b\\', b'',self.raw).decode('utf8','replace')
        return re.split(r'\x1b\[H\x1b\[(?:0|2)J',text)[-1]
    def wait(self, text=None, frame=False, timeout=20):
        start=len(self.frames);deadline=time.monotonic()+timeout
        while time.monotonic()<deadline:
            self.read()
            screen=self.screen();prompt=text and any(word in text for word in ['Local display filter','Graph depth','Graph vertices','Graph edges','View graph (Kitty):'])
            if (text is None or text in screen) and ('working / Esc cancels' not in screen or prompt) and (not frame or len(self.frames)>start) and (not frame or self.placements): return screen
            if self.proc.poll() is not None: break
        raise RuntimeError(f'PTY wait {text}, frame={frame}, exit={self.proc.poll()}\n{self.screen()[-6000:]}')
    def key(self,key,text=None,frame=True):
        count=len(self.frames);self.send(key);deadline=time.monotonic()+20
        while time.monotonic()<deadline:
            self.read()
            screen=self.screen()
            if (text is None or text in screen) and 'working / Esc cancels' not in screen and (not frame or len(self.frames)>count) and (not frame or self.placements): return screen
            if self.proc.poll() is not None:break
        raise RuntimeError(f'key {key!r} timeout\n{self.screen()[-5000:]}')
    def stop(self,name,exit=0,key='q'):
        if self.proc.poll() is None:self.send(key)
        deadline=time.monotonic()+35
        while self.proc.poll() is None and time.monotonic()<deadline:self.read()
        for _ in range(4):self.read()
        try:
            assert self.proc.poll()==exit,(name,self.proc.poll(),exit,self.screen()[-4000:])
            assert not self.images and not self.placements,(name,self.images,self.placements)
            if not args.expect_placement_failure:
                assert self.foreign_image_preserved,(name,'full-screen erase deleted unrelated graphics')
                assert not self.erased_uploaded_images,(name,'uploaded image erased before placement')
            assert b'\x1b[?25h' in self.raw,name
            safe=re.sub(rb'(\x1b_G[^;]*;)[^\x1b]+(\x1b\\)',rb'\1<base64 omitted>\2',self.raw)
            (out/(name+'.ansi')).write_bytes(safe)
            if self.frames:(out/(name+'.png')).write_bytes(self.frames[0])
            results.append(dict(name=name,exit=exit,probes=self.probes,rendered_frames=len(self.frames),owned_cleanup=True,foreign_image_preserved=self.foreign_image_preserved,erased_uploaded_images=self.erased_uploaded_images,placement_failures=self.placement_failures))
        finally:
            if self.proc.poll() is None:self.proc.kill()
            os.close(self.master)

try:
    if args.expect_placement_failure:
        s=Session(['view','model:requirement:ack']);s.stop('clear-after-upload-reproduction',6,key='')
        assert s.erased_uploaded_images>0 and s.placement_failures
        (out/'placement-reproduction.json').write_text(json.dumps(dict(status='reproduced',package=str(package),terminal_model='Kitty ED2 clears image storage; placement requires a retained image',cases=results),indent=2)+'\n')
        print(json.dumps(dict(status='reproduced',reason='ENOENT: image not found after ED2')));raise SystemExit(0)
    if args.asset_failure_client:
        s=Session(['view','model:requirement:ack']);s.stop('missing-worker',6,key='');assert 'GraphicsAsset' in s.screen()
        client=str(args.asset_failure_client.resolve())
        for command in [['--help'],['info','--json'],['diagram','model:requirement:ack','--output',str(out/'ordinary.html'),'--format','html','--json']]:
            r=subprocess.run(prefix+[client,'--workspace',str(package/'models/workspace-no-history.json'),*command],cwd=out,env=env,capture_output=True,timeout=30)
            assert r.returncode==0 and b'\x1b_G' not in r.stdout+r.stderr,(command,r.returncode,r.stdout,r.stderr)
        assert semantic()==before
        (out/'asset-failure.json').write_text(json.dumps(dict(status='pass',typed_graphics_failure=True,help_text_json_html_available=True,cases=results),indent=2)+'\n')
        print(json.dumps(dict(status='pass',asset_failure_isolated=True)));raise SystemExit(0)
    for model,root,projection in [('fruittrucks','model:requirement:ack','relationships'),('fruittrucks','model:journey:change','journey'),('fruittrucks','model:lifecycle:offer','lifecycle'),('equipment','model:entity:Booking','relationships')]:
        name=f'{model}-{projection}'
        s=Session(['--model',model,'view',root,'--projection',projection])
        s.wait('View graph (Kitty)',True)
        for key in ['n','p','e','\r','e','\t','j','k','\x1b[6~','\x1b[5~','\x1b[C','+','-','0']:s.key(key)
        s.key('?', 'keyboard help',False);assert not s.placements
        s.key('?','View graph (Kitty)')
        s.key('s','selected_source');s.key('\r','media_type')
        s.rows,s.cols=12,60;fcntl.ioctl(s.master,0x80087467,struct.pack('HHHH',12,60,1440,900));os.kill(s.proc.pid,signal.SIGWINCH)
        s.wait('Window too small');assert not s.placements
        s.rows,s.cols=36,150;fcntl.ioctl(s.master,0x80087467,struct.pack('HHHH',36,150,1440,900));os.kill(s.proc.pid,signal.SIGWINCH)
        s.wait('View graph (Kitty)',True);s.stop(name)
    # Browser state survives the viewer; late terminal replies stay filtered.
    s=Session(['browse']);s.wait('groups / type');s.send('f');s.wait('Local display filter');s.send('=Requirement\r');s.wait('filter =Requirement');s.send(' ');s.wait('instances');s.send('f');s.wait('Local display filter');s.send('R8\r');s.wait('filter R8');s.send('\r');s.wait('record · page')
    original=s.screen().splitlines()[1:4];assert s.probes==0;s.send('V')
    for prompt,value in [('View graph (Kitty):','relationships'),('Graph depth','1'),('Graph vertices','60'),('Graph edges','120')]:s.wait(prompt);s.send(value+'\r')
    s.wait('View graph (Kitty)',True);s.key('+');s.key('q','retained exact browser selection',False)
    assert s.screen().splitlines()[1:4]==original,(original,s.screen())
    s.send('\x1b_Gi=1;OK\x1b\\\x1b[4;900;1440t');s.key('j',frame=False);s.stop('browser-return')
    # Each interaction rechecks live policy, including help/resize/source prompts.
    config=package/'models/workspace-no-history.json';saved=config.read_bytes()
    for key in ['+','?','s','resize']:
        s=Session(['view','model:requirement:ack']);s.wait('View graph (Kitty)',True)
        policy=json.loads(saved);policy['models'][0]['grants']=['Open'];config.write_text(json.dumps(policy))
        if key=='resize':
            s.cols=149;fcntl.ioctl(s.master,0x80087467,struct.pack('HHHH',36,149,1440,900));os.kill(s.proc.pid,signal.SIGWINCH);s.wait('Denied')
        else:s.key(key,'Denied',False)
        assert not s.placements and 'model:requirement:ack' not in s.screen()
        config.write_bytes(saved);s.key('?','explicitly retries',False);assert not s.placements
        s.key('R','View graph (Kitty)');s.stop('revocation-'+key)
    for mode in ['unsupported','no-response','device-only','truncated','geometry-missing','ack-negative','ack-missing']:
        s=Session(['view','model:requirement:ack'],mode)
        if mode.startswith('ack-'):s.wait('explicitly retries');s.stop(mode,6)
        else:s.stop(mode,6,key='')
    s=Session(['view','model:requirement:ack']);s.wait('View graph (Kitty)',True);s.stop('ctrl-c',130,key='\x03')
    s=Session(['view','model:requirement:ack']);s.wait('View graph (Kitty)',True);s.stop('escape',0,key='\x1b')
    s=Session(['view','model:requirement:ack'],'no-response')
    deadline=time.monotonic()+20
    while not s.probes and time.monotonic()<deadline:s.read()
    assert s.probes;s.stop('early-cancel',0,key='q')
    s=Session(['view','model:requirement:ack'],'truncated')
    deadline=time.monotonic()+20
    while not s.probes and time.monotonic()<deadline:s.read()
    assert s.probes;s.stop('truncated-ctrl-c',130,key='\x03')
    s=Session(['view','model:requirement:ack']);s.wait('View graph (Kitty)',True);os.kill(s.proc.pid,signal.SIGTERM);s.stop('sigterm',130,key='')
    s=Session(['view','model:requirement:ack']);s.wait('View graph (Kitty)',True);os.kill(s.proc.pid,signal.SIGTSTP)
    deadline=time.monotonic()+10
    while time.monotonic()<deadline:
        s.read();pid,status=os.waitpid(s.proc.pid,os.WUNTRACED|os.WNOHANG)
        if pid and os.WIFSTOPPED(status):break
    else:raise RuntimeError('viewer did not suspend')
    assert not s.images and not s.placements
    os.kill(s.proc.pid,signal.SIGCONT);s.wait('View graph (Kitty)',True);s.stop('suspend-resume')
    for root in ['missing:selected-id','Open acknowledgement policy']:
        s=Session(['view',root]);s.stop('root-'+('missing' if root.startswith('missing') else 'ambiguous'),5,key='');assert s.probes==0
    s=Session(['view','model:requirement:ack','--vertices','1']);s.wait('PARTIAL',True);s.stop('partial',3)
    assert semantic()==before,'semantic records, refs, source/reference bytes or original receipts changed'
    for argv in [['view','root','--json'],['view'],['view','root','--vertices','201'],['view','root','--projection','bpmn'],['view','root']]:
        r=subprocess.run(prefix+[str(package/'kh'),'--workspace','/missing/workspace.json',*argv],cwd=out,env=env,capture_output=True,timeout=10)
        assert r.returncode==2 and b'\x1b' not in r.stdout+r.stderr,(argv,r.returncode,r.stdout,r.stderr)
    (out/'graphics.json').write_text(json.dumps(dict(status='pass',compiled=True,terminal='synthetic PTY protocol responder; no native image display claim',package=str(package),semantic_files=len(before),semantic_before_after_equal=True,no_history_required=True,isolated=bool(prefix),cases=results),indent=2)+'\n')
    print(json.dumps(dict(status='pass',cases=len(results),evidence=str(out))))
finally:
    if 'saved' in globals():config.write_bytes(saved)
    if 's' in globals() and s.proc.poll() is None:s.proc.kill()
