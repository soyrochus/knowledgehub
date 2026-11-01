# Sole-API safety coverage

The former harness inventory is in starting-test-inventory.json. Current cases below
retain their safety guarantees through the only API. They are behavioural positive,
negative, fault, independent-oracle and resource-lifetime cases, not a historical
backend/version matrix. Focused execution is recorded in stage1–3 logs; the final
checked manifest must match each named case to its observed passing native output.

| Guarantee | Current cases |
| --- | --- |
| Canonical intent, authored bytes and actor-bound retry | `canonical_retries_bytes_actor_and_operation_order`, `canonical_rfc8785_subset_vectors`, `original_retry_absence_orphans_missing_receipts_and_point_read_work` |
| Current scope policy, default denial, revocation and policy failure | `default_denial_revocation_and_policy_failure`, `recheck_policy_inside_publication_lock`, `registration_is_atomic_recursive_guarded_and_currently_authorised` |
| One-head atomic publication and invalid candidate preservation | `invalid_current_changes_preserve_head_independently`, `registration_is_atomic_recursive_guarded_and_currently_authorised`, `every_declared_signature_rejects_wrong_direction_and_targets` |
| Exact/absence guards, same-base and graph/baseline conflicts | `explicit_revision_and_absence_guards`, `separately_opened_instances_race_and_later_stale_conflict`, `typed_same_base_root_and_baseline_race_has_one_winner`, `two_process_same_base_race_has_one_winner` |
| Exact source bytes, digests, anchors, binary ranges and ownership | `binary_originals_and_every_locator_kind_resolve_and_reject_bad_quotes`, `damaged_typed_binary_is_integrity_failure_and_never_operation_absence`, `invalid_baseline_closures_and_competing_ownership_are_rejected`, `nested_owner_paths_distinguish_literal_slashes_and_tildes`, `ranges_skip_unrelated_content_and_scrub_finds_retained_corruption` |
| Immutable successors, retained selection/baselines and original outcomes | `coherent_successors_baselines_and_replay_current_thread`, `coherent_successors_baselines_and_replay_multithreaded`, `baseline_comparison_reports_unicode_additions_removals_and_stable_order`, `selection_restores_exact_history_and_refuses_a_second_origin`, `original_retry_absence_orphans_missing_receipts_and_point_read_work` |
| Process interruption, lost response, pre/post-publication uncertainty | `sigkill_typed_artifact_and_baseline_at_every_boundary`, `injected_typed_publication_and_cancellation_on_both_runtimes` |
| Cancellation retains publication resources, capacity and drain | `injected_typed_publication_and_cancellation_on_both_runtimes`, `cancelled_wait_keeps_capacity_and_drain_waits` |
| Host runtime, scheduler configurations and exclusive creation | `public_walkthrough_current_thread`, `public_walkthrough_multithreaded`, `file_adapter_requires_the_host_runtime_and_creates_none`, `coordinated_exclusive_scope_creation`, `refuses_unsupported_declared_filesystem` |
| Current metadata refusal and damaged-state non-recreation | `superseded_formats_are_refused_without_mutation`, `incompatible_and_damaged_stores_are_not_recreated` |
| Current compaction, reconstruction and retained closure | `public_shared_walkthrough_pack_reindex_and_materialise`, `sigkill_pack_install_and_cleanup_at_every_boundary`, `injected_maintenance_errors_preserve_original_state`, `coordinated_reader_and_maintenance_on_both_host_runtimes`, `frames_are_strict_bounded_lossless_and_kind_separated` |
| Authority/intent/pin-bound finite cursors and shared capacity | `bounded_navigation_cursors_policy_and_selection_binding`, `finite_cursor_pool_full_intent_and_idle_expiry_are_shared_with_query`, `composite_cursors_share_capacity_expire_and_recheck_disclosure`, `directional_kernel_traversal_keeps_cycles_budgets_and_old_pages_exact` |
| Independent semantics, finite traversal, selective work and future links | `current_query_order_filters_and_paging_match_independent_oracle`, `current_query_follows_six_hops_without_a_format_switch`, `generic_registered_nested_native_and_parallel_facts_match_independent_oracle`, `late_links_exact_scopes_and_ready_point_reads`, `ready_lookup_ignores_unrelated_artifacts_and_reports_no_state_materialisation`, `mixed_seven_hop_plan_cycles_parallel_origins_and_nearer_targets`, `limits_count_filtered_and_duplicate_work_and_bound_every_response` |
| Full ontology properties, schemas, rules, gaps and evidence distinctions | `every_declared_property_has_independent_missing_and_invalid_evidence`, `closed_constraint_primitives_have_independent_positive_and_negative_cases`, `every_family_has_a_complete_connected_typed_specimen`, `each_family_retains_an_explicit_valid_draft_and_gaps_never_waive_integrity`, `native_decision_tables_remain_exact_and_definitions_do_not_become_observations` |
| Derived cache integrity, quotas, cold/warm metadata and immutability | `large_packed_index_is_bounded_selective_and_immutable`, `publication_faults_quota_and_sidecar_reconstruction_are_safe`, `corrupt_derived_data_rebuilds_but_authoritative_corruption_is_integrity`, `readers_coordinate_cache_eviction_on_both_runtime_configurations`, `preparation_budgets_quota_and_view_eviction_preserve_knowledge` |

Removed obligations are the successful older-format reopen/frozen hashes and seed-only
bootstrap/type/version selection. Unsupported older metadata is covered by a negative
non-mutating refusal, without carrying its old decoder. Current qualified receipts,
full schemas and ordinary examples replace the separate seed envelope.

An original-operation result depends on the current authoritative operation/receipt
paths. Missing required receipt or operation-root data never becomes false absence;
unrelated artifact damage remains visible through exact reads/full scrub. This is the
current selective-read contract, not a demand to rescan all historical artifacts for
every point read. Full retained closure and historical baseline integrity are verified
separately by scrub/compaction and reopen cases.

Existing Rust 1.99.1 and the host-owned runtime are used. Platform compliance is the
user's assumption; native observations and process-failure coverage do not establish
power-loss, remote-adapter, minimum-compiler execution or human acceptance.
