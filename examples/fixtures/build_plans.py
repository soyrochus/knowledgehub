#!/usr/bin/env python3
"""Maintained fixture authoring, not production import inference.

App-specific identifiers, counts and source interpretations belong here. The
native tool consumes the resulting neutral plan without these assumptions.
Generated gaps deliberately preserve unresolved structured formalization.
"""
import argparse, copy, hashlib, json, mimetypes, posixpath, re, subprocess
from urllib.parse import unquote
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
CATALOGUE = json.loads((ROOT / 'src/ontology/catalogue.json').read_text())
TYPES = {t['name']: t for t in CATALOGUE['types']}
FORMAT = 'knowledgehub-authoring/1'
def dump(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')
def exact(name): return dict(model='sdo', id='sdo:type:'+name, revision='catalogue-3')
def token(key): return {'$ref': key}
def val(kind, value=None): return {'kind':kind, **({'value':value} if value is not None else {})}
def ref(key): return val('Ref', token(key))
def text(value): return val('Text', value)
def enum(value): return val('Enum', value)
def listing(keys): return val('List', [ref(k) for k in keys])
def setting(keys): return val('Set', [ref(k) for k in sorted(keys)])
def content(data): return dict(sha256=hashlib.sha256(data).hexdigest(), length=len(data))

class Model:
    def __init__(self, folder, label):
        self.folder, self.label = folder, label
        self.records, self.gaps, self.items, self.queries = [], [], [], []
        self.anchor_keys, self.bykey = {}, {}
        self.sources = {str(p.relative_to(folder/'source')):p.read_bytes()
                        for p in sorted((folder/'source').rglob('*')) if p.is_file()}
        self.scope = dict(hub='fixture:'+label, model='fixture:'+label, branch='main', owner='fixture:design-owner')
    def exact(self, key): return dict(model=self.scope['model'], id='model:'+key, revision='1')
    def native(self, key, kind, data, provenance=(), capture=None):
        assert key not in self.bykey, key
        record=dict(key=key, type_ref=exact(kind), data=dict(kind=kind if kind not in
            {'SourceAnchor','ElementTypeDefinition','RelationshipTypeDefinition','ConstraintDefinition','ProfileDefinition','OntologyDefinition','RepresentationBinding','SystemModel'} else
            {'SourceAnchor':'Anchor','ElementTypeDefinition':'Type','RelationshipTypeDefinition':'Relationship','ConstraintDefinition':'Constraint','ProfileDefinition':'Profile','OntologyDefinition':'Ontology','RepresentationBinding':'Representation','SystemModel':'Model'}[kind], data=data),
            provenance=list(provenance), capture=list(capture) if capture is not None else None)
        self.records.append(record);self.bykey[key]=record;return key
    def anchor(self, source, start=0, end=None):
        data=self.sources[source];end=len(data) if end is None else end
        assert 0<=start<end<=len(data)
        key=(source,start,end)
        if key not in self.anchor_keys:
            name='anchor-unit:'+hashlib.sha256(f'{source}:{start}:{end}'.encode()).hexdigest()[:20]
            self.native(name,'SourceAnchor',dict(source=token('capture:'+source), label=f'{source} bytes {start}:{end}',
                locator=dict(kind='ByteRange',start=start,end=end,quote=data[start:end].decode())),[])
            self.anchor_keys[key]=name
        return self.anchor_keys[key]
    def narrative(self, source, anchor): return val('Narrative',dict(document=token('capture:'+source),fragment=token(anchor)))
    def add(self, kind, key, name, source, span=None, properties=None):
        anchor=self.anchor(source,*(span or (0,None)));narr=self.narrative(source,anchor)
        p={'name':text(name),'description':narr, **(properties or {})}
        for field, schema in TYPES[kind]['properties'].items():
            if schema['required'] and field not in p:
                k=schema['value']
                if k['kind']=='Narrative':p[field]=narr
                elif k==dict(kind='Structured',schema='Body'):p[field]=val('Record',{'form':enum('description'),'description':narr})
                else:self.gap(key,'/'+field,anchor, f'The exact source section is retained; structured {kind}.{field} has not been established. Human meaning review and further formalization remain pending.')
        def owners(p):
            out=[]
            def walk(path,v):
                out.append(dict(path=path,owner='Document' if v['kind']=='Narrative' else 'Graph'))
                if v['kind']=='Record':
                    for n,x in v['value'].items():walk(path+'/'+n.replace('~','~0').replace('/','~1'),x)
                if v['kind'] in ('List','Set'):
                    for i,x in enumerate(v['value']):walk(path+'/'+str(i),x)
            for k,v in p.items():walk('/'+k,v)
            return out
        self.native(key,kind,dict(properties=p,owners=owners(p)),[anchor])
        self.bykey[key]['data']['kind']='Element'
        return key
    def gap(self,key,path,anchor,reason):
        self.gaps.append(dict(subject=token(key),path=path,reason=reason,provenance=[token(anchor)]))
    def edge(self,key,source,verb,from_,to,anchor_source,qualifiers=None):
        anchor=self.anchor(anchor_source)
        self.native(key,'RelationshipAssertion',dict(signature=dict(model='sdo',id=f'sdo:relationship:{source}.{verb}',revision='catalogue-3'),
            **{'from':token(from_),'to':token(to)},qualifiers=qualifiers or {}),[anchor])
        self.bykey[key]['data']['kind']='Assertion';return key
    def obligation(self,id,source,key,span=None,path='/data/data/properties/description'):
        anchor=self.anchor(source,*(span or (0,None)))
        self.items.append(dict(id=id,source=source,disposition='mapped',reason=None,
            targets=[dict(key=key,path=path,expected=None,anchor=anchor)]))
    def table(self, source, pattern):
        data=self.sources[source];s=data.decode()
        for m in re.finditer(pattern,s,re.M):
            yield m, (len(s[:m.start()].encode()),len(s[:m.end()].encode()))
    def sections(self,source,pattern):
        s=self.sources[source].decode();matches=list(re.finditer(pattern,s,re.M))
        for i,m in enumerate(matches):
            end=matches[i+1].start() if i+1<len(matches) else len(s)
            yield m, (len(s[:m.start()].encode()),len(s[:end].encode()))
    def basic(self, source):
        self.native('system','SystemModel',dict(purpose='Source-bound authored '+self.label+' design; no application execution claimed',owner=self.scope['owner'],
            ontology=dict(model='sdo',id='sdo:ontology:system-definition',revision='catalogue-3'),profile=dict(model='sdo',id='sdo:profile:system-definition',revision='catalogue-3')),[self.anchor(source)])
        self.add('Actor','actor','Authorized participant',source,properties={'actor_kind':enum('human'),'system_model':ref('system')})
    def query(self,name,kind,keys):
        self.queries.append(dict(name=name,kind='query',query=dict(selection={'selection':'Snapshot','reference':'prepared-selection-replaced-by-verifier'},type_ref=exact(kind),include_subtypes=False,
            ids=[],starts=[],signatures=[],direction='Forward',depth=1,page_size=1000,subject_limit=10000,edge_limit=10000,preparation=dict(allow_rebuild=True,owner_limit=10000,fact_limit=100000,byte_limit=67108864)),expected_ids=sorted('model:'+k for k in keys)))
    def inverse_subject(self,subject,lifecycle):
        self.queries.append(dict(name='Derived inverse lifecycle subject',kind='neighbours',query=dict(
            selection=dict(selection='Snapshot',reference='replaced-by-verifier'),starts=[token(subject)],direction='Incoming',
            selectors=[dict(kind='Property',definition=exact('LifecycleDefinition'),path='/subject')],categories=['Property'],opposite_type=exact('LifecycleDefinition'),include_subtypes=True,
            fact_scope='Selected',projection='Records',page_size=100,examined_limit=10000,byte_limit=262144,preparation=dict(allow_rebuild=True,owner_limit=10000,fact_limit=100000,byte_limit=67108864)),expected_ids=['model:'+lifecycle]))
    def property_check(self,name,start,definition,path,direction,expected):
        self.queries.append(dict(name=name,kind='neighbours',query=dict(
            selection=dict(selection='Snapshot',reference='replaced-by-verifier'),starts=[token(start)],direction=direction,
            selectors=[dict(kind='Property',definition=exact(definition),path=path)],categories=['Property'],opposite_type=None,include_subtypes=False,
            fact_scope='Selected',projection='Records',page_size=100,examined_limit=10000,byte_limit=262144,preparation=dict(allow_rebuild=True,owner_limit=10000,fact_limit=100000,byte_limit=67108864)),expected_ids=sorted('model:'+k for k in expected)))
    def finish(self):
        # Every file is accounted independently, including auxiliary research and renditions.
        input_rows=[]
        for path,data in self.sources.items():
            binary=path.endswith(('.pdf','.png'));metadata=Path(path).name=='.DS_Store'
            authority=('prototype-regression-design; not executed evidence' if '/tests/' in path else 'working-ui' if path.startswith('mockups/') else 'research-simulated-or-supporting' if '/research/' in path else 'authored-design')
            dependencies=[]
            if path.endswith(('.html','.css','.js')):
                s=data.decode();links=re.findall(r'(?:src|href)=[\"\']([^\"\'#]+)|url\([\"\']?([^\)\"\']+)',s)
                for pair in links:
                    link=next((x for x in pair if x),'').split('?',1)[0]
                    if not link or ':' in link or link.startswith(('#','data:','/')):continue
                    normalized=str((Path(path).parent/link))
                    # Resolve dot segments only in fixture interpretation; tool inputs stay normalized.
                    parts=[]
                    for part in normalized.split('/'):
                        if part=='..':
                            if parts:parts.pop()
                        elif part!='.':parts.append(part)
                    dependencies.append('/'.join(parts))
            input_rows.append(dict(key=path,path=path,media_type=mimetypes.guess_type(path)[0] or 'application/octet-stream',authority=authority,status='hypothesis' if '/research/' in path else 'authored-requirement',rendition_group=path.rsplit('.',1)[0] if path.endswith(('.md','.pdf')) else None,dependencies=sorted(set(dependencies)),exclude_reason='macOS Finder metadata; not design semantics' if metadata else None,expected=content(data)))
            if metadata:continue
            if binary:
                self.binary(path,data)
            else:
                key='file-definition:'+path
                self.add('Requirement',key,'Retained source definition: '+path,path,properties={'claim_kind':enum('authored-requirement'),'rationale':listing([])})
                self.obligation('file:'+path,path,key)
                # Script/UI details outside the bounded graph vocabulary remain
                # individually inspectable source obligations, rather than being
                # inferred as implemented application behavior. The full original
                # is the representation; each locator selects the observed construct.
                if path.endswith(('.html','.js','.css')):
                    source=data.decode();observations=[]
                    patterns={
                        'control':r'<(?:button|input|select|textarea|a)\b[^>]*>',
                        'function':r'\bfunction\s+[A-Za-z][A-Za-z0-9_]*\s*\([^\n]*',
                        'event':r'addEventListener\([^\n]*',
                        'observable':r'(?:notify|showToast|status|openDialog|confirm)\([^\n]*',
                        'dispatch-alias':r'(?:name|action)\s*===\s*[\x27\x22][^\x27\x22]+[\x27\x22]',
                    }
                    for category,pattern in patterns.items():
                        for match in re.finditer(pattern,source,re.I if category=='control' else 0):
                            span=(len(source[:match.start()].encode()),len(source[:match.end()].encode()))
                            anchor=self.anchor(path,*span)
                            if anchor not in self.bykey[key]['provenance']:self.bykey[key]['provenance'].append(anchor)
                            identity=f'ui-artifact:{path}:{category}:{span[0]}'
                            self.obligation(identity,path,key,span)
                            observations.append(dict(id=identity,category=category,source=path,start=span[0],end=span[1],target=key,formalization='exact original and locator; no execution or inferred binding'))
                    if observations:
                        self.ui_observations=getattr(self,'ui_observations',[])+observations
        assert len({i['id'] for i in self.items})==len(self.items)
        self.folder.mkdir(parents=True,exist_ok=True)
        owners={"product":"specs/memento-spec.md","domain":"specs/memento-domain-spec.md","journeys-stories-scenarios":"specs/memento-experience-spec.md","engineering-acceptance-contracts":"specs/memento-technical-spec.md","selected-platform":"specs/memento-platform-spec.md","visible-shell":"mockups/mockups.js","working-ui":"mockups/atlas-board.js"} if self.label=='memento' else {'design':'design.md'}
        reading_order=['specs/memento-spec.md','specs/memento-experience-spec.md','specs/memento-domain-spec.md','specs/memento-technical-spec.md','specs/memento-platform-spec.md'] if self.label=='memento' else ['design.md']
        precedence=reading_order+[r['key'] for r in input_rows if not r['exclude_reason'] and r['key'] not in reading_order]
        dump(self.folder/'source-manifest.json',dict(format=FORMAT,scope=self.scope,inputs=input_rows,
            authority=dict(field_owners=owners,precedence=precedence,unresolved_conflicts=[],review_status='agent-reviewed supplied ownership hierarchy and source locators; human meaning acceptance pending')))
        dump(self.folder/'model-plan.json',dict(format=FORMAT,catalogue=dict(model='sdo',id='sdo:ontology:system-definition',revision='catalogue-3'),profile=dict(model='sdo',id='sdo:profile:system-definition',revision='catalogue-3'),records=self.records,gaps=self.gaps,baseline='imported-A',selection=getattr(self,'selection',None)))
        missing=set()
        for path,data in self.sources.items():
            if not path.endswith('.md'):continue
            for link in re.findall(r'\]\(([^)]+)\)',data.decode()):
                link=unquote(link.split('#',1)[0])
                if not link or ':' in link:continue
                target=posixpath.normpath(posixpath.join(posixpath.dirname(path),link))
                if target not in self.sources:missing.add(f'{path} -> {target}: linked context not supplied in this package; no implementation, evidence or additional definition inferred')
        source_gaps=['No application implementation, provider execution or acceptance is established. Structured fields left unresolved have exact native gaps; original narrative/artifacts remain authoritative.','Auxiliary research and prototype simulations are captured with their original status.','Visible interaction details beyond the declared typed bindings remain exact source artifacts with individual construct locators; source accounting does not claim graph-complete behavior formalization.']+sorted(missing)
        dump(self.folder/'coverage.json',dict(format=FORMAT,items=self.items,queries=self.queries,interpretation_review='agent-reviewed exact source identities, authority and construct locators; human meaning acceptance pending',source_gaps=source_gaps))
        dump(self.folder/'source-map.json',{'format':FORMAT,'items':self.items})
        dump(self.folder/'queries.json',{'format':FORMAT,'queries':self.queries})
        dump(self.folder/'ui-source-review.json',{'format':FORMAT,'review':'agent inspection of retained source constructs; human meaning acceptance pending','observations':getattr(self,'ui_observations',[]),'limit':'Exact artifacts retain branches, confirmations, failures, permissions, callbacks and styling beyond the bounded structured bindings. These source locators are not proof of graph-complete formalization or application execution.'})
        return dict(records=len(self.records),gaps=len(self.gaps),obligations=len(self.items))
    def binary(self,path,data):
        if path.endswith('.pdf'):
            proc=subprocess.run(['pdftotext','-layout',str(self.folder/'source'/path),'-'],capture_output=True,check=True)
            derived=proc.stdout
            version=subprocess.run(['pdftotext','-v'],capture_output=True,text=True).stderr.splitlines()[0]
            pages=derived.decode().split('\f');pages=pages[:-1] if not pages[-1].strip() else pages
            entries={};offset=0
            for i,page in enumerate(pages):
                b=page.encode()
                if page.strip():entries[f'page-{i+1}']=dict(kind='PdfPage',label=f'page {i+1}',location=dict(kind='PdfPage',page=i+1,pages=len(pages)),derived=self.exact('extraction:'+path),start=offset,end=offset+len(b))
                offset+=len(b)+1
        else:
            # Inspected static design illustration; this is an authored visual interpretation.
            derived=b'Static Memento illustration with navigation, collaborative board, evidence, decisions, proposed alternatives and an agent activity panel. Historical illustration includes sync wording and purple accents; working HTML/CSS/JS owns current UI behavior. It is not implementation or execution evidence.'
            import struct
            width,height=struct.unpack('>II',data[16:24])
            version='authored visual inspection 2026-11-03'
            entries={'image':dict(kind='ImageRegion',label='whole retained illustration',location=dict(kind='ImageRegion',x=0,y=0,width=width,height=height,image_width=width,image_height=height),derived=self.exact('extraction:'+path),start=0,end=len(derived))}
        self.native('extraction:'+path,'SourceRevision',dict(source=token('source:'+path),original=content(derived),media_type='text/plain',derived_from=token('capture:'+path),derivation=version),[],derived)
        manifest=dict(source=self.exact('capture:'+path),original=content(data),tool='pdftotext' if path.endswith('.pdf') else 'authored-visual-inspection',version=version,entries=entries)
        manifest_bytes=json.dumps(manifest,ensure_ascii=False).encode()
        self.native('locator:'+path,'SourceRevision',dict(source=token('source:'+path),original=content(manifest_bytes),media_type='application/json',derived_from=token('capture:'+path),derivation='source-bound locator inventory'),[],manifest_bytes)
        for entry,value in entries.items():
            a='binary-anchor:'+path+':'+entry
            self.native(a,'SourceAnchor',dict(source=token('capture:'+path),label=value['label'],locator=dict(kind='Manifest',locator=value['kind'],manifest=token('locator:'+path),entry=entry,quote=None)),[])
            key='binary-definition:'+path+':'+entry
            # Native representation retains a precise binary-source binding.
            self.native(key,'RepresentationBinding',dict(subject=token('system'),anchor=token(a),role='OriginalEvidence'),[a])
            self.items.append(dict(id='file:'+path+':'+entry,source=path,disposition='mapped',reason=None,targets=[dict(key=key,path='/data/data/anchor',expected=None,anchor=a)]))

def experience(m,source):
    journeys=[];story_journeys={}
    for match,span in m.sections(source,r'^### (UJ-\d+) - (.+)$'):
        key=match[1];journeys.append(key)
        m.add('Journey',key,match[2],source,span,{'actor':ref('actor')});m.obligation(key,source,key,span)
    for match,span in m.sections(source,r'^### (US-\d+) - (.+)$'):
        key=match[1];n=int(key[3:]);chunk=m.sources[source][span[0]:span[1]].decode()
        declared=sorted(set(re.findall(r'UJ-\d+',chunk)))
        story_journeys[n]=declared or journeys
        # Bound each promise and each scenario independently to exact original bytes.
        m.add('UserStory',key,match[2],source,span,{'actor':ref('actor')});m.obligation(key,source,key,span)
        for journey in story_journeys[n]:
            step='step:'+journey+':'+key
            m.add('JourneyStep',step,match[2],source,span,{'journey':ref(journey)})
            m.edge('journey-step:'+journey+':'+key,'Journey','containsStep',journey,step,source)
            m.edge('story-step:'+journey+':'+key,'UserStory','supportsStep',key,step,source)
        requirement='R-'+key[3:]
        a=re.search(r'\*\*Requirement '+requirement+r':\*\* (.+?)\*\*Trace:',chunk,re.S);assert a,requirement
        rspan=(span[0]+len(chunk[:a.start(1)].encode()),span[0]+len(chunk[:a.end(1)].encode()))
        m.add('Requirement',requirement,requirement,source,rspan,{'claim_kind':enum('authored-requirement'),'rationale':listing([])})
        m.obligation(requirement,source,requirement,rspan)
        m.edge('formalises:'+key,'Requirement','formalises',requirement,key,source)
        for letter in 'ab':
            scenario=f'S-{n:02}{letter}'
            a=re.search(r'\*\*'+scenario+r':\*\* (.+?)(?=\*\*S-|\n\n|$)',chunk,re.S);assert a,scenario
            sspan=(span[0]+len(chunk[:a.start(1)].encode()),span[0]+len(chunk[:a.end(1)].encode()))
            narr=m.narrative(source,m.anchor(source,*sspan))
            m.add('Scenario',scenario,scenario,source,sspan,{'expected':val('List',[narr])});m.obligation(scenario,source,scenario,sspan)
            m.edge('illustrates:'+scenario,'Scenario','illustrates',scenario,requirement,source)
    # Actual four-column product map across all existing journeys; no invented journey.
    map_span=next(span for match,span in m.sections(source,r'^## (\d+)\. (.+)$') if match[1]=='3')
    columns=['Enter and resume','Think together','Capture and consult','Decide, act and remember']
    grouping=[list(range(1,10)),list(range(10,19))+[36],list(range(19,29)),list(range(29,36))]
    placements=[]
    for index,(name,stories) in enumerate(zip(columns,grouping)):
        activity='activity:'+str(index)
        tasks=[f'task:{index}:US-{n:02}' for n in stories]
        if index==1:tasks.append('task:1:US-32')
        scope=sorted({j for n in stories+([32] if index==1 else []) for j in story_journeys[n]})
        m.add('UserActivity',activity,name,source,map_span,{'journeys':setting(scope),'tasks':listing(tasks)})
        for order,n in enumerate(stories):
            task=f'task:{index}:US-{n:02}'
            m.add('UserTask',task,m.bykey[f'US-{n:02}']['data']['data']['properties']['name']['value'],source,map_span,{'activity':ref(activity)})
            key=f'placement:{index}:US-{n:02}';placements.append(key)
            m.add('StoryPlacement',key,f'US-{n:02} under {name}',source,map_span,{'map':ref('story-map'),'task':ref(task),'story':ref(f'US-{n:02}'),'order':val('Integer','0')})
    # Reuse a story in its action-review task as the source P3 column explicitly does.
    m.add('UserTask','task:1:US-32','Review the exact Action',source,map_span,{'activity':ref('activity:1')})
    m.add('StoryPlacement','placement:1:US-32','US-32 exact Action review',source,map_span,{'map':ref('story-map'),'task':ref('task:1:US-32'),'story':ref('US-32'),'order':val('Integer','0')});placements.append('placement:1:US-32')
    m.add('StoryMap','story-map','Product-wide P1–P5 story map',source,map_span,{'journeys':setting(journeys),'activities':listing(['activity:'+str(i) for i in range(4)]),'placements':listing(placements),'cuts':listing(['P'+str(i) for i in range(1,6)])})
    cumulative=set()
    for match,span in m.table(source,r'^\| (P[1-5]): ([^\n]+)\|$'):
        key=match[1];row=match[0]
        nums=set(int(n) for n in re.findall(r'US-(\d+)',row))
        for a,b in re.findall(r'US-(\d+) to (?:US-)?(\d+)',row):nums.update(range(int(a),int(b)+1))
        for start,tail in re.findall(r'US-(\d+)((?:/\d+)+)',row):nums.update([int(start)]+[int(n) for n in tail.split('/') if n])
        if key=='P5':nums.update(range(1,37))
        cumulative.update(p for p in placements if int(p.rsplit('US-',1)[1]) in nums)
        m.add('OutcomeCut',key,key+': '+match[2].split('|')[0].strip(),source,span,{'map':ref('story-map'),'placements':setting(cumulative),'prerequisites':listing([] if key=='P1' else ['P'+str(int(key[1])-1)]),'exclusions':val('List',[])})
        m.obligation(key,source,key,span)
    for dep,pre in [(6,1),(3,5),(9,6),(20,19),(21,20),(22,20),(25,24),(26,24),(30,29),(32,31),(33,31)]:
        m.edge(f'depends:{dep}:{pre}','UserStory','dependsOn',f'US-{dep:02}',f'US-{pre:02}',source,{'dependency_kind':enum('required'),'condition':val('Null')})
    # Source explicitly offers alternatives, not an unconditional dependency.
    m.edge('depends:27:28','UserStory','dependsOn','US-27','US-28',source,{'dependency_kind':enum('conditional'),'condition':m.narrative(source,m.anchor(source,*map_span))})
    m.query('all-journeys','Journey',journeys);m.query('all-stories','UserStory',[f'US-{n:02}' for n in range(1,37)])

def memento():
    m=Model(BASE/'memento','memento');domain='specs/memento-domain-spec.md';experience_source='specs/memento-experience-spec.md';technical='specs/memento-technical-spec.md';product='specs/memento-spec.md'
    m.basic(product);experience(m,experience_source)
    contexts=[]
    for match,span in m.sections(domain,r'^## ([3-7])\. (.+)$'):
        key='context:'+match[2];contexts.append(key)
        m.add('BoundedContext',key,match[2],domain,span,{'model_owner':text('Memento authored domain definition'),'language':listing([]),'hypotheses':listing([])})
        m.obligation(key,domain,key,span)
    m.query('five-contexts','BoundedContext',contexts)
    # All explicit domain value/record declarations, not only representative Action.
    headers={'Value','Aggregate','Type','ID','Record'}
    seen=set()
    for match,span in m.table(domain,r'^\| ([A-Z][A-Za-z]+) \| ([^\n]+)\|$'):
        name=match[1]
        if name in headers or name in seen:continue
        seen.add(name)
        kind='ValueObjectType' if span[0] < m.sources[domain].find(b'## 3.') else 'EntityType'
        key=('value:' if kind=='ValueObjectType' else 'entity:')+name
        m.add(kind,key,name,domain,span,{'fields':listing([])})
        m.obligation('definition:'+name,domain,key,span)
    for source,prefix,kind in [(domain,'DR','Invariant'),(product,'FA','Requirement'),(technical,'AC','AcceptanceCriterion')]:
        ids=[]
        for match,span in m.table(source,rf'^\| ({prefix}-\d+) \| ([^\n]+)\|$'):
            key=match[1];ids.append(key)
            p={'claim_kind':enum('authored-requirement'),'rationale':listing([])} if kind=='Requirement' else {'valid_examples':listing(['S-32a']) if key=='DR-09' else listing([]),'invalid_examples':listing(['S-32b']) if key=='DR-09' else listing([])} if kind=='Invariant' else {'scope':listing(['system']),'decision_owner':text('Source-defined authorized acceptance owner; no acceptance recorded')}
            m.add(kind,key,key,source,span,p);m.obligation(key,source,key,span)
        assert len(ids)=={'DR':26,'FA':11,'AC':30}[prefix],(prefix,len(ids))
    # Published commands/use cases preserve their exact source table contracts.
    commands={}
    for match,span in m.table(domain,r'^\| ((?:WA|CK|VC|AP|CW)-(?:Q)?\d+) ([^|]+) \| ([^\n]+)\|$'):
        id=match[1];names=[n.strip() for n in match[2].split('/')]
        uc='usecase:'+id;m.add('UseCase',uc,match[2],domain,span,{'actor':listing(['actor'])});m.obligation('usecase:'+id,domain,uc,span)
        for name in names:
            kind='QueryDefinition' if '-Q' in id else 'CommandDefinition';key='command:'+name
            if key in m.bykey:continue
            m.add(kind,key,name,domain,span);commands[name]=key
            if kind=='CommandDefinition':m.edge('handler:'+name,'CommandDefinition','handledBy',key,uc,domain)
    m.edge('approval-satisfies','UseCase','satisfies','usecase:CW-08','R-32',experience_source)
    m.add('FunctionDefinition','function:dispatch-authority','Validate exact approved dispatch',domain,properties={'rules':listing(['DR-09']),'reads':listing(['entity:Action'])})
    m.edge('dispatch-invokes','UseCase','invokes','usecase:CW-08','function:dispatch-authority',domain)
    m.edge('dispatch-operates','FunctionDefinition','operatesOn','function:dispatch-authority','aggregate:Action',domain)
    for id in ['CW-07','CW-08']:
        rule=m.add('Invariant','invariant:'+id,'Exact human Action authority '+id,domain)
        m.edge('approval-invariant:'+id,'AggregateDefinition','protects','aggregate:Action',rule,domain)
    # Source's Action subject, explicit logical targets and lifecycle.
    action_span=next(span for match,span in m.sections(domain,r'^### (\d+\.\d+) (.+)$') if match[1]=='7.2')
    m.add('EntityType','entity:Action','Action',domain,action_span,{'fields':listing(['field:Action.status','field:Action.prerequisites'])})
    m.add('AggregateDefinition','aggregate:Action','Action consistency boundary',domain,action_span,{'members':listing(['entity:Action'])})
    m.edge('action-root','AggregateDefinition','hasRoot','aggregate:Action','entity:Action',domain)
    states=['AwaitingApproval','Approved','Rejected','Queued','InExecution','Succeeded','Failed','Uncertain']
    m.add('DataSchema','schema:Action','Action definition schema',domain,action_span,{'fields':listing(['field:Action.status','field:Action.prerequisites']),'identity_fields':listing([])})
    m.add('DataType','datatype:ActionState','Action state enumeration',domain,action_span,{'value_kind':enum('enumeration'),'enum_values':val('Set',[text(s) for s in sorted(states)]),'value_constraints':listing([])})
    m.add('DataType','datatype:ActionRef','Action prerequisite target',domain,action_span,{'value_kind':enum('reference'),'reference_targets':setting(['entity:Action']),'value_constraints':listing([])})
    m.add('DataType','datatype:ActionRefs','Action prerequisite collection',domain,action_span,{'value_kind':enum('set'),'item_type':ref('datatype:ActionRef'),'value_constraints':listing([])})
    for name,dt in [('status','datatype:ActionState'),('prerequisites','datatype:ActionRefs')]:
        m.add('FieldDefinition','field:Action.'+name,name,domain,action_span,{'schema':ref('schema:Action'),'datatype':ref(dt),'nullable':val('Bool',False),'constraints':listing([])})
    # Identity format is not invented from a UI label.
    links=[];raw=m.sources[domain][action_span[0]:action_span[1]].decode()
    for i,(a,b) in enumerate(re.findall(r'^  (\w+) --> (\w+)',raw,re.M)):
        key='transition:Action:'+str(i);links.append(key)
        m.add('Transition',key,a+' -> '+b,domain,action_span,{'owner':ref('lifecycle:Action'),'from':ref('state:Action:'+a),'to':ref('state:Action:'+b)})
        m.gap(key,'/triggeredBy',m.anchor(domain,*action_span),'Source transition triggers remain in the exact lifecycle description; an exact Command/Event trigger binding is not yet resolved.')
    for state in states:m.add('State','state:Action:'+state,state,domain,action_span,{'owner':ref('lifecycle:Action'),'terminal':val('Bool',state in ['Succeeded','Rejected'])})
    m.add('LifecycleDefinition','lifecycle:Action','Action lifecycle',domain,action_span,{'subject':ref('entity:Action'),'states':listing(['state:Action:'+s for s in states]),'initial_state':ref('state:Action:AwaitingApproval'),'terminal_states':listing(['state:Action:Succeeded','state:Action:Rejected']),'transitions':listing(links)})
    m.inverse_subject('entity:Action','lifecycle:Action')
    # Actual six template definitions, no fictional accepted Decisions or provider records.
    templates=[]
    for match,span in m.table(experience_source,r'^\| ([a-z]+(?:-[a-z]+)+) / ([^|]+) \| ([^\n]+)\|$'):
        key='template:'+match[1];templates.append(key);m.add('ComponentDefinition',key,match[2].strip(),experience_source,span);m.obligation(key,experience_source,key,span)
    assert len(templates)==6
    # Working surfaces and every explicit HTML control plus script command branch.
    surfaces=[]
    for path in sorted(p for p in m.sources if re.match(r'mockups/0[1-5]-.*\.html$',p)):
        screen='surface:'+Path(path).stem;surfaces.append(screen)
        state='view:'+Path(path).stem
        m.add('ViewState',state,'Source-defined visible states',path,properties={'screen':ref(screen)})
        m.add('ScreenDefinition',screen,Path(path).stem,path,properties={'states':listing([state])});m.obligation(screen,path,screen)
        s=m.sources[path].decode()
        for i,match in enumerate(re.finditer(r'<(?:button|input|select|textarea|a)\b[^>]*>',s,re.I)):
            span=(len(s[:match.start()].encode()),len(s[:match.end()].encode()));key=f'control:{path}:{i}'
            m.add('Interaction',key,match[0],path,span,{'gesture':text('Source-declared control activation')});m.obligation(key,path,key,span)
    assert len(surfaces)==5;m.query('five-working-surfaces','ScreenDefinition',surfaces)
    js='mockups/atlas-board.js';s=m.sources[js].decode()
    action_start=s.index('function action(');action_end=s.index('function fit()',action_start)
    action_source=s[action_start:action_end]
    for match in re.finditer(r"name === '([^']+)'",action_source):
        # The whole dispatcher preserves compound conditions, guards and effects.
        # Literal aliases in a shared branch are separate stable obligations.
        start=action_start;end=action_end
        span=(len(s[:start].encode()),len(s[:end].encode()));key='interaction:'+match[1]
        if key in m.bykey:continue
        m.add('Interaction',key,match[1],js,span,{'gesture':text('Explicit source-defined '+match[1]+' action')});m.obligation(key,js,key,span)
    functions=list(re.finditer(r'^[ \t]*function ([A-Za-z][A-Za-z0-9]*)\(',s,re.M))
    for i,match in enumerate(functions):
        end=functions[i+1].start() if i+1<len(functions) else len(s)
        span=(len(s[:match.start()].encode()),len(s[:end].encode()));key='ui-function:'+match[1]
        if key in m.bykey:continue
        m.add('FunctionDefinition',key,'Working UI '+match[1],js,span)
        m.obligation(key,js,key,span)
    for action,command in [('approve','ApproveAction'),('execute','ExecuteApprovedAction')]:
        key='binding:'+action
        m.add('ActionBinding',key,action+' exact action binding',js,properties={'interaction':ref('interaction:'+action),'command':ref(commands[command])})
        m.edge('issues:'+action,'Interaction','issues','interaction:'+action,commands[command],js)
    # Whole exact behavior code is retained; these observations claim no script execution.
    m.native('working-ui-binding','RepresentationBinding',dict(subject=token('interaction:execute'),anchor=token(m.anchor(js)),role='OriginalEvidence'),[m.anchor(js)])
    m.obligation('approval-code-artifact',js,'working-ui-binding',path='/data/data/anchor')
    # Native assertion traversal checks the actual promise/example/command/UI chain.
    sigs=[dict(model='sdo',id='sdo:relationship:'+s,revision='catalogue-3') for s in ['Requirement.formalises','Scenario.illustrates','UseCase.satisfies','CommandDefinition.handledBy','Interaction.issues']]
    expected=['R-32','S-32a','S-32b','usecase:CW-08','command:ExecuteApprovedAction','interaction:execute','illustrates:S-32a','illustrates:S-32b','approval-satisfies','handler:ExecuteApprovedAction','issues:execute']
    q=copy.deepcopy(m.queries[0]['query']);q.update(type_ref=None,starts=[token('R-32')],signatures=sigs,direction='Reverse',depth=5)
    m.queries.append(dict(name='R-32 exact source approval trace',kind='query',query=q,expected_ids=sorted('model:'+k for k in expected)))
    q=copy.deepcopy(q);q.update(signatures=[dict(model='sdo',id='sdo:relationship:Requirement.formalises',revision='catalogue-3')],direction='Forward',depth=2)
    m.queries.append(dict(name='R-32 formalises shared US-32',kind='query',query=q,expected_ids=sorted('model:'+k for k in ['R-32','US-32','formalises:US-32'])))
    # Planned architecture comes from the authored technical design, never invented files.
    modules=[]
    for ctx in contexts:
        key='module:'+ctx;modules.append(key)
        m.add('Module',key,ctx,technical,properties={'implementation_status':enum('planned'),'component':ref('software:Memento')})
    m.add('SoftwareComponent','software:Memento','Memento modular monolith',technical,properties={'modules':listing(modules)})
    for key,name in [('port:storage','Selected storage port'),('port:provider','Scoped provider port')]:m.add('Port',key,name,technical)
    for name in ['SQLite','PostgreSQL']:
        m.add('Adapter','adapter:'+name,name+' planned application store',technical,properties={'implementation_status':enum('planned'),'port':ref('port:storage')})
    for kind in ['StoryMap','StoryPlacement','OutcomeCut','Invariant','AcceptanceCriterion','ComponentDefinition','ActionBinding','Interaction','FunctionDefinition','DataType','FieldDefinition','Module','Adapter','Transition','State']:
        keys=[r['key'] for r in m.records if r['type_ref']==exact(kind)]
        m.query('exact inventory '+kind,kind,keys)
    for cut in ['P1','P2','P3','P4','P5']:
        record=m.bykey[cut];expected=record['data']['data']['properties']['placements']
        span=next(span for match,span in m.table(experience_source,r'^\| (P[1-5]): ([^\n]+)\|$') if match[1]==cut)
        m.obligation('exact-cut-membership:'+cut,experience_source,cut,span,path='/data/data/properties/placements')
        m.items[-1]['targets'][0]['expected']=expected
    m.property_check('DR-09 dispatch function','DR-09','FunctionDefinition','/rules/*','Incoming',['function:dispatch-authority'])
    m.property_check('Execute binding exact interaction','binding:execute','ActionBinding','/interaction','Outgoing',['interaction:execute'])
    m.property_check('Execute binding exact command','binding:execute','ActionBinding','/command','Outgoing',['command:ExecuteApprovedAction'])
    return m, m.finish()

def unrelated():
    folder=BASE/'unrelated-design';(folder/'source').mkdir(parents=True,exist_ok=True)
    source='design.md'
    (folder/'source'/source).write_text('''# Equipment reservation design\nA borrower reserves an Asset, then returns to inspect a Booking. Booking retains one Asset reference, state Requested/Confirmed/Cancelled and the original booking reason. A planned notification adapter has no implementation artifact. Cancellation and a two-hour waiting policy are descriptions, never timers executed by KnowledgeHub. A shared story appears in two tasks and cumulative cuts. Reference-format and implementation choices remain explicitly unresolved.\n''')
    (folder/'source'/'rule.js').write_text("// Harmless retained sentinel; importing must never call this entrypoint.\nexport function bookingPolicy() { throw new Error('STORED_RULE_MUST_NOT_EXECUTE'); }\n")
    m=Model(folder,'equipment');m.basic(source)
    for id,name in [('reserve','Reserve equipment'),('return','Return to the booking')]:m.add('Journey','journey:'+id,name,source,properties={'actor':ref('actor')})
    m.add('UserStory','story:book','Reserve an available Asset',source,properties={'actor':ref('actor')})
    activities=[];placements=[]
    for i,name in enumerate(['Find equipment','Review booking']):
        task='task:'+str(i);act='activity:'+str(i);placement='placement:'+str(i);activities.append(act);placements.append(placement)
        m.add('UserActivity',act,name,source,properties={'journeys':setting(['journey:reserve','journey:return']),'tasks':listing([task])})
        m.add('UserTask',task,name,source,properties={'activity':ref(act)})
        m.add('StoryPlacement',placement,name,source,properties={'map':ref('map'),'task':ref(task),'story':ref('story:book'),'order':val('Integer','0')})
    m.add('StoryMap','map','Reservation map',source,properties={'journeys':setting(['journey:reserve','journey:return']),'activities':listing(activities),'placements':listing(placements),'cuts':listing(['cut:first','cut:later'])})
    for id in ['first','later']:m.add('OutcomeCut','cut:'+id,id,source,properties={'map':ref('map'),'placements':setting(placements if id=='later' else placements[:1]),'prerequisites':listing(['cut:first'] if id=='later' else []),'exclusions':val('List',[])})
    for entity in ['Asset','Booking']:m.add('EntityType','entity:'+entity,entity,source,properties={'fields':listing([])})
    m.add('DataSchema','schema:Booking','Booking logical record',source,properties={'fields':listing(['field:asset']),'identity_fields':listing([])})
    m.add('DataType','datatype:AssetRef','Asset reference',source,properties={'value_kind':enum('reference'),'reference_targets':setting(['entity:Asset']),'value_constraints':listing([])})
    m.add('DataType','datatype:AssetRefs','Asset references',source,properties={'value_kind':enum('list'),'item_type':ref('datatype:AssetRef'),'value_constraints':listing([])})
    m.add('DataType','datatype:BookingRecord','Booking record',source,properties={'value_kind':enum('record'),'record_schema':ref('schema:Booking'),'value_constraints':listing([])})
    m.add('DataType','datatype:BookingState','Booking states',source,properties={'value_kind':enum('enumeration'),'enum_values':val('Set',[text(x) for x in sorted(['Requested','Confirmed','Cancelled'])]),'value_constraints':listing([])})
    m.add('FieldDefinition','field:asset','asset',source,properties={'schema':ref('schema:Booking'),'datatype':ref('datatype:AssetRef'),'reference_targets':setting(['entity:Asset']),'nullable':val('Bool',False),'constraints':listing([])})
    states=['Requested','Confirmed','Cancelled']
    for name in states:m.add('State','state:'+name,name,source,properties={'owner':ref('lifecycle:Booking'),'terminal':val('Bool',name=='Cancelled')})
    m.add('LifecycleDefinition','lifecycle:Booking','Booking lifecycle',source,properties={'subject':ref('entity:Booking'),'states':listing(['state:'+s for s in states]),'initial_state':ref('state:Requested'),'terminal_states':listing(['state:Cancelled']),'transitions':listing([])})
    m.inverse_subject('entity:Booking','lifecycle:Booking')
    m.add('Adapter','adapter:notification','Planned notification adapter',source,properties={'implementation_status':enum('planned')})
    m.add('BusinessRule','rule:booking','Descriptive booking policy','rule.js',properties={'rationale':listing([]),'rule':val('Record',{'form':enum('native'),'artifact':val('Artifact',token('capture:rule.js')),'language':text('javascript'),'entrypoint':text('bookingPolicy')})})
    for key in ['map','entity:Asset','entity:Booking','lifecycle:Booking','field:asset','datatype:AssetRef','adapter:notification']:
        m.obligation(key,source,key)
    m.query('booking-lifecycle','LifecycleDefinition',['lifecycle:Booking'])
    # Governed, application-independent extension hierarchy and signature.
    anchor=m.anchor(source);citation=dict(original=content(m.sources[source]),section='Authored reusable extension fixture',source_status='synthetic design',refinement='General inherited reference/endpoint contract, not a built-in Booking type')
    parent='extension:IdentifiedDefinition';child='extension:SpecialDefinition'
    m.native(parent,'ElementTypeDefinition',dict(name='IdentifiedDefinition',family='Generic extension fixture',coverage='fixture:extension-base',native=False,parents=[exact('EntityType')],properties={'related_subject':dict(value=dict(kind='Reference',schema=[dict(kind='Type',selection=exact('EntityType'))]),required=True,nullable=False,owner='Graph')},constraints=[],citation=citation),[anchor])
    m.native(child,'ElementTypeDefinition',dict(name='SpecialDefinition',family='Generic extension fixture',coverage='fixture:extension-child',native=False,parents=[token(parent)],properties={},constraints=[],citation=citation),[anchor])
    signature='extension:associatedSubject'
    m.native(signature,'RelationshipTypeDefinition',dict(name='associatedSubject',coverage='fixture:extension-signature',**{'from':[dict(kind='Type',selection=token(parent))],'to':[dict(kind='Type',selection=exact('EntityType'))]},min_complete=0,max=None,acyclic=False,qualifiers={},citation=citation),[anchor])
    ontology='extension:ontology';profile='extension:profile'
    relationships=[]
    for relation in CATALOGUE['relationships']:
        sel=relation['from'][0];first=sel['selection']['id'].removeprefix('sdo:type:') if sel['kind']=='Type' else sel['selection']
        relationships.append(dict(model='sdo',id='sdo:relationship:'+first+'.'+relation['name'],revision='catalogue-3'))
    vocabulary=[exact(t['name']) for t in CATALOGUE['types']]+[token(parent),token(child)]
    m.native(profile,'ProfileDefinition',dict(name='generic-extension-fixture',ontology=token(ontology),vocabulary=vocabulary,allow_gaps=True,required_types=[],inapplicable={},citation=citation),[anchor])
    m.native(ontology,'OntologyDefinition',dict(name='Generic extension fixture ontology',version='fixture-extension/1',types=vocabulary,relationships=relationships+[token(signature)],constraints=[dict(model='sdo',id=f'sdo:constraint:V-{i:02}',revision='catalogue-3') for i in range(1,18)],profiles=[token(profile)],structures=CATALOGUE['structures'],citation=citation),[anchor])
    m.add('EntityType','extension:instance','Special identified subject',source,properties={'fields':listing([]),'related_subject':ref('entity:Asset')})
    m.bykey['extension:instance']['type_ref']=token(child)
    # Plan type_ref is exact, so resolve the fixture's own preallocated schema token.
    m.bykey['extension:instance']['type_ref']=token(child)
    m.native('extension:edge','RelationshipAssertion',dict(signature=token(signature),**{'from':token('extension:instance'),'to':token('entity:Asset')},qualifiers={}),[anchor]);m.bykey['extension:edge']['data']['kind']='Assertion'
    m.query('exact application definitions','EntityType',['entity:Asset','entity:Booking'])
    q=copy.deepcopy(m.queries[-1]);q['name']='inclusive application definitions';q['query']['include_subtypes']=True;q['expected_ids'].append('model:extension:instance');q['expected_ids'].sort();m.queries.append(q)
    m.selection=(ontology,profile)
    return m,m.finish()

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('fixture',choices=['memento','unrelated-design','all'],default='all',nargs='?');args=parser.parse_args()
    results={}
    if args.fixture in ['memento','all']:results['memento']=memento()[1]
    if args.fixture in ['unrelated-design','all']:results['unrelated-design']=unrelated()[1]
    print(json.dumps(results))
