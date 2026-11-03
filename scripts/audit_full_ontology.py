#!/usr/bin/env python3
"""Reconcile source denominators with public consumer results and observed native checks."""
import argparse,collections,hashlib,json,pathlib,re
from host_sources import change_dir
root=pathlib.Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser();p.add_argument('catalogue');p.add_argument('model');p.add_argument('input');p.add_argument('--log',default='delivery/evidence/full-ontology-run.txt');p.add_argument('--output',default='delivery/evidence/full-ontology-coverage.json');a=p.parse_args()
load=lambda path:json.loads(pathlib.Path(path).read_text())
catalogue=load(a.catalogue);model=load(a.model);capture=load(a.input)
data=load(root/'src/ontology/catalogue.json');log=(root/a.log).read_text()
passed=set(re.findall(r'^test ([\w:]+) \.\.\. ok$',log,re.M))
def evidence(*names):
 for name in names:assert any(t==name or t.endswith('::'+name) for t in passed),'unexecuted evidence: '+name
 return list(names)
def key(ref):return (ref['model'],ref['id'],ref['revision'])
digests={key(ref):sha for ref,sha in catalogue['definition_digests']}
records={key(r['reference']):r for r in catalogue['specimens']}
rows={};families=collections.Counter();field_count=0
inventory=(change_dir(root,'express-full-ontology-and-self-model')/'ontology-coverage.md').read_text()
expected=set(re.findall(r'^\| ([KTRP]\d+) \|',inventory,re.M))
for definition in data['types']:
 ref=('sdo','sdo:type:'+definition['name'],'catalogue-3');assert ref in digests
 row=dict(definition=dict(zip(('model','id','revision'),ref)),sha256=digests[ref],source=definition['citation'])
 if definition['native']:
  actual=catalogue['kernel'][definition['name']];row['materialisation']=actual['reference'];row['evidence']=evidence('kernel_codecs_are_closed_and_missing_catalogue_family_is_reported','all_families_and_kernel_relationships_commit_and_reopen')
 else:
  candidates=[r for r in records.values() if key(r['type_ref'])==ref];assert candidates,definition['name']
  sample=next(r for r in candidates if r['reference']['id']=='specimen:'+definition['name']);props=sample['data']['data']['properties']
  required={n for n,rule in definition['properties'].items() if rule['required']}
  assert required<=props.keys() and props.keys()<=definition['properties'].keys(),definition['name']
  assert len(props)>2,'name-only instance: '+definition['name']
  assert all(isinstance(v,dict) and 'kind' in v for v in props.values()),'unchecked properties'
  row.update(specimen=sample['reference'],specimen_sha256=sample['sha256'],properties=sorted(props),required=sorted(required),evidence=evidence('every_declared_property_has_independent_missing_and_invalid_evidence','each_family_retains_an_explicit_valid_draft_and_gaps_never_waive_integrity','all_families_and_kernel_relationships_commit_and_reopen'))
  families[definition['family']]+=1;field_count+=len(required)
 rows[definition['coverage']]=row
assert len(catalogue['kernel'])==17
assert catalogue['kernel']['ElementIdentity']['reference']['revision']=='@identity'
assert catalogue['kernel']['ElementRevision']['reference']==next(r['reference'] for r in records.values() if r['reference']['id']=='specimen:Requirement')
for definition in data['relationships']:
 ref=('sdo','sdo:relationship:'+(definition['from'][0]['selection']['id'].removeprefix('sdo:type:') if definition['from'][0]['kind']=='Type' else definition['from'][0]['selection'])+'.'+definition['name'],'catalogue-3');assert ref in digests
 matches=[r for r in records.values() if r['data']['kind']=='Assertion' and key(r['data']['data']['signature'])==ref];assert matches,definition['coverage']
 rows[definition['coverage']]=dict(definition=dict(zip(('model','id','revision'),ref)),sha256=digests[ref],specimens=[r['reference'] for r in matches],source=definition['citation'],evidence=evidence('every_declared_signature_rejects_wrong_direction_and_targets','all_families_and_kernel_relationships_commit_and_reopen'))
assert {k for k in rows if not k.startswith('E')}==expected and len(expected)==183
assert {k for k in rows if k.startswith('E')}=={'E01','E02','E03'}
assert len(families)==14 and sum(families.values())==129
rules={
 'V-01':['scalar_encodings_closed_requests_and_revision_tokens_are_lossless','coherent_successors_baselines_and_replay_current_thread'],
 'V-02':['every_declared_property_has_independent_missing_and_invalid_evidence'],
 'V-03':['every_declared_signature_rejects_wrong_direction_and_targets'],
 'V-04':['invalid_baseline_closures_and_competing_ownership_are_rejected','selection_restores_exact_history_and_refuses_a_second_origin'],
 'V-05':['binary_originals_and_every_locator_kind_resolve_and_reject_bad_quotes','damaged_typed_binary_is_integrity_failure_and_never_operation_absence'],
 'V-06':['invalid_baseline_closures_and_competing_ownership_are_rejected','nested_owner_paths_distinguish_literal_slashes_and_tildes'],
 'V-07':['every_family_has_a_complete_connected_typed_specimen','every_declared_signature_rejects_wrong_direction_and_targets'],
 'V-08':['specialised_integrity_draft_gaps_and_native_roles','typed_same_base_root_and_baseline_race_has_one_winner'],
 'V-09':['every_declared_property_has_independent_missing_and_invalid_evidence','native_decision_tables_remain_exact_and_definitions_do_not_become_observations'],
 'V-10':['cycles_process_membership_transformation_kinds_and_contracts'],
 'V-11':['specialised_integrity_draft_gaps_and_native_roles'],
 'V-12':['cycles_process_membership_transformation_kinds_and_contracts'],
 'V-13':['specialised_integrity_draft_gaps_and_native_roles','invalid_baseline_closures_and_competing_ownership_are_rejected'],
 'V-14':['every_declared_signature_rejects_wrong_direction_and_targets'],
 'V-15':['native_decision_tables_remain_exact_and_definitions_do_not_become_observations'],
 'V-16':['specialised_integrity_draft_gaps_and_native_roles'],
 'V-17':['each_family_retains_an_explicit_valid_draft_and_gaps_never_waive_integrity','retained_definitions_do_not_hide_a_missing_active_family'],
 'S01':['native_decision_tables_remain_exact_and_definitions_do_not_become_observations'],
 'S02':['specialised_integrity_draft_gaps_and_native_roles'],
 'S03':['cycles_process_membership_transformation_kinds_and_contracts'],
 'S04':['cycles_process_membership_transformation_kinds_and_contracts'],
 'S05':['every_declared_property_has_independent_missing_and_invalid_evidence','specialised_integrity_draft_gaps_and_native_roles'],
 'S06':['registration_is_atomic_recursive_guarded_and_currently_authorised','every_declared_signature_rejects_wrong_direction_and_targets'],
 'S07':['every_declared_property_has_independent_missing_and_invalid_evidence','every_family_has_a_complete_connected_typed_specimen'],
 'S08':['native_decision_tables_remain_exact_and_definitions_do_not_become_observations','coherent_successors_baselines_and_replay_multithreaded'],
}
for rule,names in rules.items():evidence(*names)
assert len(rules)==25
assert model['structurally_valid'] and not model['purpose_complete']
assert not model['Q01']['missing'] and model['actual_human_assessments']==0
assert len(model['Q08']['inapplicable'])==2 and model['Q08']['gaps']
assert {'Q01','Q02','Q03','Q04','Q05','Q06','Q07','Q08'}<=model.keys()
assert all(model['answers'][q] for q in ('Q02','Q03','Q04','Q05'))
assert model['Q07']['A']!=model['Q07']['B'] and model['Q07']['changed']
assert any(r['id']=='kh:run:captured-native-conformance' for r in model['Q06']['records'])
for file in capture['files']:assert model['source_originals'][file['path']]==file['reference']
refs={key(ref) for values in model['groups'].values() for ref in values}
ids={ref[1] for ref in refs}
# Owning-document items must expand to distinct records, rather than one node per group.
origins=load(root/'examples/self_model_support/origins.json')
def namespace(path):
 return origins['public_namespaces'].get(path) or 'knowledgehub::internal::'+origins['modules'][path]

for unit in capture['units']:
 if unit['kind']=='rule':assert 'kh:rule:'+unit['name']+':BusinessRule' in ids
 if unit['kind']=='decision':assert 'kh:decision:'+unit['name'] in ids
 if unit['kind']=='slice':assert 'kh:slice:'+unit['name'] in ids
 if unit['kind']=='story':assert 'kh:story:'+unit['name'] in ids
 if unit['kind'] in ('contract','native-contract') and unit['path'].startswith('src/'):
  assert 'kh:schema:'+namespace(unit['path'])+'::'+unit['name'] in ids,unit['name']
  for field in unit['fields']:assert 'kh:field:'+namespace(unit['path'])+'::'+unit['name']+'::'+field['name'] in ids,(unit['name'],field['name'])
 if unit['kind']=='constant' and unit['path'].startswith('src/'):assert 'kh:constant:'+unit['name'] in ids
for group in ('purpose','participants','domain','contexts','language','consistency','rules','requirements','journeys','behaviour','operations','information','transformations','architecture','policy','decisions','operation','delivery','verification','configuration','historical-evidence'):assert model['groups'].get(group),group
assertions={r['reference']['id']:r for r in model['assertions']}
for name in ['kh:assertion:capability-goal','kh:assertion:journey-outcome','kh:assertion:commit-handler','urn:assertion:commit-satisfies-atomic-change','kh:assertion:commit-invokes','kh:assertion:admit-aggregate','kh:assertion:aggregate-root','kh:assertion:aggregate-invariant','kh:assertion:crate-implements','kh:assertion:adapter-translates','kh:assertion:policy-governs','kh:assertion:interruption-checks','kh:assertion:run-executes','kh:assertion:evidence-criterion','kh:assertion:source-supports','kh:assertion:admission-represents']:assert name in assertions,name
sha=lambda path:hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
output=dict(format='knowledgehub-coverage-audit/2',catalogue=dict(rows=rows,families=families,required_properties=field_count,kernel=17,family_types=129,source_signatures=32,prose_signatures=6,experience_refinements=['E01','E02','E03'],original_source_rows=183),obligations={r:dict(evidence=names) for r,names in rules.items()},self_model=dict(groups={k:len(v) for k,v in model['groups'].items()},source_units=len(capture['units']),original_files=len(capture['files']),selected_records=model['selected_records'],retained_records=model['retained_records'],explicit_gaps=len(model['Q08']['gaps']),purpose_complete=False,human_assessments=0,questions=['Q%02d'%n for n in range(1,9)],required_assertions=16),inputs={path:sha(path) for path in (a.catalogue,a.model,a.input)},scope='Observed structural conformance and public walkthrough. Semantic adequacy and human acceptance remain separate.')
out=root/a.output;assert not out.exists(),'fresh audit output required; historical evidence is not overwritten';out.write_text(json.dumps(output,indent=2)+'\n');print(json.dumps(dict(audit=str(out.relative_to(root)),catalogue_rows=len(rows),obligations=len(rules),groups=len(model['groups']),explicit_gaps=len(model['Q08']['gaps']))))
