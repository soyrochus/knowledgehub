#!/usr/bin/env python3
"""Host inventory only: capture exact working-tree bytes and source locations, never backend internals."""
import argparse,ast,datetime,hashlib,json,mimetypes,pathlib,re,subprocess,sys
sys.path.insert(0,str(pathlib.Path(__file__).resolve().parents[1]/"scripts"))
sys.dont_write_bytecode=True
from host_sources import source_paths,checked_run,load_origins,package_identity
ROOT=pathlib.Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser();p.add_argument('--output',default='target/self-model-input.json');p.add_argument('--log',default='delivery/evidence/full-ontology-run.txt');p.add_argument('--checked-manifest');p.add_argument('--derived',default='target/ontology-pdf-extracted.txt');a=p.parse_args()
def digest(b):return {'sha256':hashlib.sha256(b).hexdigest(),'length':len(b)}
paths=[x for x in source_paths(ROOT) if 'fixtures' not in x.parts]
paths.append(ROOT/a.log)
if a.checked_manifest:
 checked=checked_run(ROOT,ROOT/a.checked_manifest,ROOT/a.log)
 paths.append(ROOT/a.checked_manifest)
else:checked=None
for output in (a.output,a.derived):
 relative=(ROOT/output).relative_to(ROOT)
 if {'old_hub','hub','orig_hub'}.intersection(relative.parts):raise ValueError('generated output cannot target retained stores')
 if relative.parts[0]!='target' and relative.parts[:3]!=('delivery','evidence','hub-rebuild'):
  raise ValueError('generated capture output must be under target/ or a run evidence directory')
origins=load_origins(ROOT)
version=package_identity(ROOT)
for path in paths:
 rel=str(path.relative_to(ROOT));parts=path.relative_to(ROOT).parts
 declared=(rel in origins['sources'] or rel in origins['modules'] or rel=='src/ontology/catalogue.json' or rel.startswith(('openspec/changes/','openspec/deferred/','openspec/specs/')) or (parts[:3]==('delivery','evidence','hub-rebuild') and len(parts)==5 and parts[-1] in origins['run_artifacts']))
 if not declared:raise ValueError('declare semantic source binding for '+rel)

for historical in ['delivery/evidence/kh-sl-001-run.txt','delivery/evidence/full-ontology-start.txt','delivery/evidence/full-ontology-start.json','delivery/evidence/full-ontology-start.tar.gz']:
 if (ROOT/historical).exists():paths.append(ROOT/historical)
paths=sorted(set(paths))
files=[];units=[]
def unit(path,kind,name,start,end,**extra):
 b=(ROOT/path).read_bytes();assert 0<=start<end<=len(b)
 units.append(dict(path=path,kind=kind,name=name,start=start,end=end,**extra))
def lines(path,text,kind,pattern):
 off=0
 for line in text.splitlines(keepends=True):
  m=re.search(pattern,line)
  if m:
   end=off+len(line.encode())
   if kind in ('slice','requirement','section'):
    level=len(line)-len(line.lstrip('#'));tail=text.encode()[end:].decode();nxt=re.search(r'(?m)^#{1,'+str(level)+r'}\s+',tail)
    end=end+len(tail[:nxt.start()].encode()) if nxt else len(text.encode())
   unit(path,kind,m.group(1),off,end,text=line.strip())
  off+=len(line.encode())
def block(text,start):
 # Rust item bodies have balanced braces; ignore strings and comments when counting.
 pos=text.find('{',start)
 if pos<0 or (text.find(';',start)>=0 and text.find(';',start)<pos):return None
 depth=0;quoted=False;escape=False;comment=False
 for i in range(pos,len(text)):
  c=text[i]
  if comment:
   if c=='\n':comment=False
   continue
  if quoted:
   if escape:escape=False
   elif c=='\\':escape=True
   elif c=='"':quoted=False
   continue
  if text.startswith('//',i):comment=True;continue
  if c=='"':quoted=True;continue
  if c=='{':depth+=1
  elif c=='}':
   depth-=1
   if depth==0:return pos,i+1
 return None
def split_top(text):
 depth=0;start=0
 for i,c in enumerate(text):
  if c in '({[<':depth+=1
  if c in ')}]>':depth-=1
  if c==',' and depth==0:
   yield text[start:i].strip();start=i+1
 if text[start:].strip():yield text[start:].strip()
for file in paths:
 rel=str(file.relative_to(ROOT));b=file.read_bytes();media='application/pdf' if file.suffix=='.pdf' else 'application/gzip' if file.suffix=='.gz' else 'text/markdown' if file.suffix=='.md' else 'text/plain'
 files.append(dict(path=rel,reference=digest(b),media_type=media))
 if file.suffix=='.pdf':continue
 try:text=b.decode()
 except UnicodeDecodeError:continue
 if file.suffix=='.md':
  lines(rel,text,'section',r'^#{1,4}\s+(.+)')
  if rel=='design/domain-analysis.md':
   lines(rel,text,'rule',r'^\| (KH-DR-\d+) \|')
   language=text.split('## 3. Ubiquitous language',1)[1].split('## 4.',1)[0]
   base=text.index(language);offset=len(text[:base].encode())
   for line in language.splitlines(keepends=True):
    cells=line.strip().split('|')
    if len(cells)>3 and cells[1].strip() not in ('Term','---',''):
     unit(rel,'term',cells[1].strip().strip('`'),offset,offset+len(line.encode()),text=line.strip())
    offset+=len(line.encode())
  if rel=='design/self-hosting-plan.md':lines(rel,text,'decision',r'^\| (KH-D-\d+) \|')
  if rel=='delivery/slice-plan.md':
   lines(rel,text,'slice',r'^### (KH-SL-\d+)')
   lines(rel,text,'journey',r'^\| (KH-J-\d+)')
   seen=set();offset=0
   for line in text.splitlines(keepends=True):
    for story in re.findall(r'KH-US-\d+',line):
     if story not in seen:unit(rel,'story',story,offset,offset+len(line.encode()),text=line.strip());seen.add(story)
    offset+=len(line.encode())
  if 'specs' in file.parts:lines(rel,text,'requirement',r'^### Requirement: (.+)')
  if file.name=='tasks.md':lines(rel,text,'task',r'^- \[[ x]\] (\d+\.\d+ .+)')
 if file.suffix=='.py':
  tree=ast.parse(text);line_offsets=[0]
  for line in text.splitlines(keepends=True):line_offsets.append(line_offsets[-1]+len(line.encode()))
  class Visitor(ast.NodeVisitor):
   def __init__(self):self.owners=[]
   def visit_ClassDef(self,node):
    self.owners.append(node.name);self.generic_visit(node);self.owners.pop()
   def visit_FunctionDef(self,node):
    name='.'.join(self.owners+[node.name]);signature=ast.unparse(node).split('\n',1)[0]
    parameters=[dict(name=x.arg,rust_type=ast.unparse(x.annotation) if x.annotation else 'Python value') for x in node.args.args if x.arg!='self']
    unit(rel,'test' if rel.startswith('tests/') and node.name.startswith('test_') else 'host-function',name,line_offsets[node.lineno-1],line_offsets[node.end_lineno],signature=signature,has_body=True,parameters=parameters,result=ast.unparse(node.returns) if node.returns else 'Python value',public=False,asynchronous=False)
    self.owners.append(node.name);self.generic_visit(node);self.owners.pop()
  Visitor().visit(tree)
 if file.suffix=='.rs':
  for m in re.finditer(r'(?m)^\s*pub\s+(struct|enum|trait)\s+(\w+)',text):
   bounds=block(text,m.end());end=bounds[1] if bounds else text.find(';',m.end())+1
   if end<=m.start():continue
   body=text[bounds[0]+1:bounds[1]-1] if bounds else ''
   fields=[]
   if m[1]=='struct':
    for part in split_top(body):
     f=re.search(r'\bpub\s+(\w+)\s*:\s*(.+)',part,re.S)
     if f:fields.append({'name':f[1],'rust_type':re.sub(r'\s+',' ',f[2]).strip()})
   if m[1]=='struct' and not bounds:
    tuple_type=re.search(r'\((.*)\)',text[m.end():end],re.S)
    if tuple_type:
     for i,typ in enumerate(split_top(tuple_type[1])):
      if typ.startswith('pub '):fields.append({'name':str(i),'rust_type':typ.removeprefix('pub ').strip()})
   if m[1]=='enum':
    for part in split_top(body):
     part=re.sub(r'#\[[^\]]*\]','',part).strip();v=re.match(r'(\w+)\s*(.*)',part,re.S)
     if v:
      fields.append({'name':v[1],'rust_type':re.sub(r'\s+',' ',v[2]).strip() or 'unit variant'})
      payload=v[2].strip()
      if payload.startswith('{'):
       for member in split_top(payload[1:-1]):
        f=re.search(r'(\w+)\s*:\s*(.+)',member,re.S)
        if f:fields.append({'name':v[1]+'.'+f[1],'rust_type':re.sub(r'\s+',' ',f[2]).strip()})
      elif payload.startswith('('):
       for i,typ in enumerate(split_top(payload[1:-1])):fields.append({'name':v[1]+'.'+str(i),'rust_type':re.sub(r'\s+',' ',typ).strip()})
   unit(rel,'contract',m[2],len(text[:m.start()].encode()),len(text[:end].encode()),rust_kind=m[1],fields=fields)
  for name in origins['native_contracts'].get(rel,[]):
   m=re.search(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?struct '+re.escape(name)+r'\b',text)
   if not m:raise ValueError('stale native contract origin: '+rel+'::'+name)
   bounds=block(text,m.end());fields=[]
   for part in split_top(text[bounds[0]+1:bounds[1]-1]):
    f=re.search(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:\s*(.+)',part,re.S)
    if f:fields.append({'name':f[1],'rust_type':re.sub(r'\s+',' ',f[2]).strip()})
   unit(rel,'native-contract',name,len(text[:m.start()].encode()),len(text[:bounds[1]].encode()),rust_kind='struct',fields=fields)
  for m in re.finditer(r'pub const (\w+)\s*:\s*([^=]+)=\s*(.*?);',text,re.S):
   literals=re.findall(r'"([^"]*)"',m[3]);value=literals if '[' in m[2] else literals[0] if literals else None
   unit(rel,'constant',m[1],len(text[:m.start()].encode()),len(text[:m.end()].encode()),fields=[{'name':'value','rust_type':m[2].strip()}],value=value)
  impls=[]
  for m in re.finditer(r'\btrait\s+(\w+)',text):
   bounds=block(text,m.end())
   if bounds:impls.append((bounds[0],bounds[1],m[1]))
  for m in re.finditer(r'\bimpl\s+(?:<[^>]*>\s*)?(\w+)(?:\s+for\s+(\w+))?',text):
   bounds=block(text,m.end())
   if bounds:impls.append((bounds[0],bounds[1],m[2] or m[1]))
  for m in re.finditer(r'(?m)^\s*(pub(?:\([^)]*\))?\s+)?(async\s+)?fn\s+(\w+)[^\n(]*\(',text):
   bounds=block(text,m.end());end=bounds[1] if bounds else text.find(';',m.end())+1
   if end<=m.start():continue
   owner=next((name for start,end,name in reversed(impls) if start<m.start()<end),'module')
   before=text[:m.start()].rstrip();test=bool(re.search(r'#\[(?:tokio::)?test(?:\([^\]]*\))?\]\s*$',before))
   signature=text[m.start():bounds[0] if bounds else end].strip()
   arg_start=signature.find('(');depth=0;arg_end=arg_start
   for i in range(arg_start,len(signature)):
    if signature[i]=='(':depth+=1
    elif signature[i]==')':
     depth-=1
     if depth==0:arg_end=i;break
   parameters=[]
   for parameter_index,arg in enumerate(split_top(signature[arg_start+1:arg_end])):
    if ':' in arg:
     name,typ=arg.split(':',1);name=name.strip();parameters.append({'name':f'argument_{parameter_index}' if name=='_' else name,'rust_type':typ.strip()})
   result=signature[arg_end+1:].split('where')[0].strip().removeprefix('->').strip() or '()'
   unit(rel,'test' if test else 'operation' if m[1]=='pub ' else 'function',owner+'.'+m[3],len(text[:m.start()].encode()),len(text[:end].encode()),signature=signature,has_body=bounds is not None,parameters=parameters,result=result,public=m[1]=='pub ',asynchronous=bool(m[2]))
# Retain actual PDF extraction separately, with page delimiters, original digest and exact derived spans.
pdf='source/system-definition-ontology-proposal.pdf';pdfbytes=(ROOT/pdf).read_bytes()
proc=subprocess.run(['pdftotext','-layout',str(ROOT/pdf),'-'],check=True,capture_output=True)
derived=ROOT/a.derived;derived.parent.mkdir(parents=True,exist_ok=True);derived.write_bytes(proc.stdout)
files.append(dict(path=a.derived,reference=digest(proc.stdout),media_type='text/plain',derived_from=pdf,derivation='pdftotext -layout; form-feed delimited physical pages'))
off=0;pages=[]
for i,page in enumerate(proc.stdout.split(b'\x0c')):
 if page.strip():pages.append(dict(index=i,start=off,end=off+len(page),label=f'physical PDF page {i+1}'))
 off+=len(page)+1
capture={'format':'knowledgehub-host-inventory/1','library_version':version,'captured_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_status':'exact working tree; modified files are not identified as Git HEAD','git_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'compiler':subprocess.check_output(['rustc','-Vv'],cwd=ROOT,text=True),'filesystem':subprocess.check_output(['findmnt','-T',str(ROOT),'-J','-o','TARGET,SOURCE,FSTYPE,OPTIONS'],text=True),'kernel':subprocess.check_output(['uname','-a'],text=True).strip(),'extractor':subprocess.run(['pdftotext','-v'],capture_output=True,text=True).stderr.strip(),'files':files,'units':units,'pdf_pages':pages,'checked':checked,'log':a.log}
out=ROOT/a.output;out.parent.mkdir(parents=True,exist_ok=True);out.write_text(json.dumps(capture,indent=2)+'\n')
print(f'{len(files)} exact files, {len(units)} source units, {len(pages)} retained PDF pages -> {out.relative_to(ROOT)}')
