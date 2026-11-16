# Tasks: Add reclaim plan kernel

## Phase 1: Model and proofs

- [x] [serial] I1 Add reachability, admission, validity, and soundness specifications. r[reclaim-plan.reachability] r[reclaim-plan.admission] r[reclaim-plan.validity]
- [x] [serial] I2 Prove `closed_set_contains_reachable` and `reachable_extend`. r[reclaim-plan.reachability]
- [x] [serial] I3 Prove `no_live_candidate`, `no_dangling_reference`, and `protected_never_candidate` from validity alone. r[reclaim-plan.no-live-candidate] r[reclaim-plan.no-dangling-reference] r[reclaim-plan.protected-closure]
- [x] [serial] I4 Add `check_plan` and `admissible` with `ensures` equal to their specifications. r[reclaim-plan.validity] r[reclaim-plan.admission]
- [x] [serial] I5 Add the fixpoint `plan` with a round cap and prove planned results valid and sound; prove `planned_result_reclaims_every_unreachable_node`. r[reclaim-plan.planner] r[reclaim-plan.no-leak]
- [x] [serial] I6 Prove with `count_true_set` and `count_true_bounded` that the planner never reaches its round cap, so admissible inputs always plan. r[reclaim-plan.planner]

- [x] [serial] I10 Prove `planned_result_retains_exactly_live`, `planned_result_is_least_valid_plan`, and `planned_results_are_unique`, with `valid_plan_retains_live`. r[reclaim-plan.exact-retention] r[reclaim-plan.least-plan]
- [x] [serial] I11 Prove `stronger_input_never_adds_candidates` and `planned_result_depends_only_on_sets`, with `reachable_edges_mono` and `live_mono`; show each hypothesis is needed by removing it. r[reclaim-plan.monotone] r[reclaim-plan.set-invariance]
- [x] [serial] I12 Add `project_keys` and `key_mask`, which validate a consumer's key-to-index mapping and plan path lists, with the lexicographic order lemmas, and prove `key_live_matches_index_live`, `no_listed_candidate_path_is_live`, and `every_live_path_is_listed_retained`; show each hypothesis and each runtime check is needed by removing it. r[reclaim-plan.key-projection]

## Phase 2: Tests

- [x] [parallel] V1 Add positive and negative tests, including corrupted plans. r[reclaim-plan.validity] r[reclaim-plan.admission]
- [x] [parallel] V2 Add an exhaustive test over every graph, root set, and reclaimable set with up to three nodes against an independent oracle. r[reclaim-plan.evidence]
- [x] [parallel] V6 Add an exhaustive keyed-graph test against the breadth-first oracle and negative tests for wrong, out-of-range, unsorted, duplicate, and mis-shaped mappings and masks. r[reclaim-plan.key-projection]
- [x] [parallel] V5 Add exhaustive tests that the planned result is the least of all 27,437 valid plans, that adding a root, reference, or protection never adds a candidate, and that reordering and duplicating inputs keep the plan. r[reclaim-plan.least-plan] r[reclaim-plan.monotone] r[reclaim-plan.set-invariance]

## Phase 3: Evidence and lifecycle

- [x] [serial] I7 Add `docs/requirements.md` entries, the module export, and non-claims. r[reclaim-plan.evidence]
- [x] [serial] I8 Add README index rows and the kernel's claim and non-claim paragraph. r[reclaim-plan.evidence]
- [ ] [serial] I9 Regenerate `docs/module-index.md` and add the invariant catalog, property index, and proof-artifact manifest entries with the repository tools. r[reclaim-plan.evidence]
- [ ] [serial] V3 Run `./scripts/verus.sh src/lib.rs` with the pinned verifier, Tracey, and the proof-gap inventory; capture transcripts. r[reclaim-plan.evidence]
- [ ] [serial] V4 Run Cairn validation and the change gates; archive. r[reclaim-plan.evidence]
