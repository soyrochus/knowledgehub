#!/usr/bin/env python3
"""Maintained, bounded teaching interpretation; not an import inference engine."""
import hashlib, importlib.util, json, shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('plans',ROOT/'examples/fixtures/build_plans.py')
p=importlib.util.module_from_spec(spec);spec.loader.exec_module(p)

def build():
    folder=BASE/'fruittrucks';m=p.Model(folder,'fruittrucks');s='design.md';d='route-change-design-notes.md'
    m.basic(s)
    for name in ['Dispatch','Delivery Execution','Customer Communication']:
        m.add('BoundedContext','context:'+name,name,s,properties={'model_owner':p.text('fixture:design-owner')})
    m.add('Requirement','requirement:ack','R8 — acknowledge an in-progress change',s,properties={'rationale':p.listing([]),'claim_kind':p.enum('authored-requirement')})
    m.add('Requirement','requirement:delivery','Q1 — instruction delivery within sixty seconds',s,properties={'rationale':p.listing([]),'claim_kind':p.enum('authored-requirement')})
    m.add('Requirement','requirement:expiry','Proposed five-minute offer expiry; production policy open',d,properties={'rationale':p.listing([]),'claim_kind':p.enum('design-decision')})
    m.add('UseCase','usecase:ack','Acknowledge exact released offer',d,properties={'actor':p.listing(['actor']),'preconditions':p.listing(['guard:current']),'errors':p.listing([])})
    m.add('UseCase','command:release','Release changed stop-order offer',d,properties={'actor':p.listing(['actor']),'preconditions':p.listing(['guard:current']),'errors':p.listing([])})
    m.add('CommandDefinition','command:acknowledge','Acknowledge exact offer command (proposed design)',d,properties={'guards':p.listing(['guard:current']),'errors':p.listing([])})
    m.gap('command:acknowledge','/handledBy',m.anchor(d),'Logical handler binding is not established by these proposed teaching notes; no executing handler is inferred.')
    for name in ['accepted','stale','expired']:
        m.add('Scenario','scenario:'+name,{'accepted':'Exact acknowledgement activates the current offer','stale':'Changed plan or progress refuses activation','expired':'Proposed expiry refuses late activation'}[name],d)
        m.edge('edge:scenario:'+name,'Scenario','illustrates','scenario:'+name,'requirement:ack',d)
    m.edge('edge:satisfies','UseCase','satisfies','usecase:ack','requirement:ack',d)
    m.edge('edge:satisfies:parallel','UseCase','satisfies','usecase:ack','requirement:ack',s)
    m.edge('edge:ack:context','ElementRevision','ownedIn','usecase:ack','context:Dispatch',s)
    m.edge('edge:offer:context','ElementRevision','ownedIn','entity:ChangeOffer','context:Dispatch',s)
    for suffix in ['authority','emergency']:
        m.add('Requirement','question:'+suffix,'Open acknowledgement policy',d,properties={'rationale':p.listing([]),'claim_kind':p.enum('hypothesis')})
    m.add('Journey','journey:change','Establish a changed stop order',d,properties={'actor':p.ref('actor')})
    acts=[]
    for i,name in enumerate(['Understand disruption','Prepare and check','Release exact offer','Acknowledge or decline','Recheck and activate','Track communication']):
        a=f'activity:{i}';t=f'task:{i}';acts.append(a)
        m.add('UserActivity',a,name,d,properties={'journeys':p.setting(['journey:change']),'tasks':p.listing([t])})
        m.add('UserTask',t,name,d,properties={'activity':p.ref(a)})
    m.add('StoryMap','storymap:change','Proposed route-change activities',d,properties={'journeys':p.setting(['journey:change']),'activities':p.listing(acts),'placements':p.listing([]),'cuts':p.listing([])})
    for entity in ['Route','ChangeOffer','Acknowledgement']:
        m.add('EntityType','entity:'+entity,entity,'route-change-information.md',properties={'fields':p.listing([])})
    m.add('Precondition','guard:current','Exact offer, unexpired, current plan, assignment and progress',d)
    states=['Draft','Awaiting acknowledgement','Activated','Declined','Expired']
    transitions=[('release','Draft','Awaiting acknowledgement'),('acknowledge','Awaiting acknowledgement','Activated'),('decline','Awaiting acknowledgement','Declined'),('proposed-expiry','Awaiting acknowledgement','Expired')]
    for name in states:m.add('State','state:'+name,name,d,properties={'owner':p.ref('lifecycle:offer'),'terminal':p.val('Bool',name in states[2:])})
    for name,from_,to in transitions:
        m.add('Transition','transition:'+name,name,d,properties={'owner':p.ref('lifecycle:offer'),'from':p.ref('state:'+from_),'to':p.ref('state:'+to),'guards':p.listing(['guard:current'] if to=='Activated' else []),'effects':p.listing([])})
        m.gap('transition:'+name,'/triggeredBy',m.anchor(d),'Trigger binding remains an explicit structured omission; the proposed transition description does not establish an executable trigger.')
    m.add('LifecycleDefinition','lifecycle:offer','Proposed offer lifecycle subset',d,properties={'subject':p.ref('entity:ChangeOffer'),'states':p.listing(['state:'+x for x in states]),'initial_state':p.ref('state:Draft'),'terminal_states':p.listing(['state:'+x for x in states[2:]]),'transitions':p.listing(['transition:'+x[0] for x in transitions])})
    for kind,key in [('Requirement','requirement:ack'),('Requirement','requirement:expiry'),('UseCase','usecase:ack'),('LifecycleDefinition','lifecycle:offer')]:m.obligation(key,s if key=='requirement:ack' else d,key)
    m.query('Acknowledgement requirement','UseCase',['usecase:ack','command:release'])
    m.inverse_subject('entity:ChangeOffer','lifecycle:offer')
    result=m.finish()
    plan=json.loads((folder/'model-plan.json').read_text())
    for record in plan['records']:
        if record['key'].startswith('file-definition:') and record['key']!='file-definition:design.md':
            record['data']['data']['properties']['claim_kind']=p.enum('design-decision')
    p.dump(folder/'model-plan.json',plan)
    manifest=json.loads((folder/'source-manifest.json').read_text())
    for row in manifest['inputs']:
        if not row.get('exclude_reason'):
            row['status']='authored-requirement' if row['key']=='design.md' else 'hypothesis'
            row['media_type']='text/markdown'
    manifest['authority']['unresolved_conflicts']=['R8 retained; manual/interview wording conflict remains unresolved. Five minutes is a proposed teaching choice, not approved production policy. Q1 measures instruction delivery, not acknowledgement.']
    p.dump(folder/'source-manifest.json',manifest)
    return result

if __name__=='__main__':print(json.dumps(build()))
