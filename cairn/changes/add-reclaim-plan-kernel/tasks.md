# Tasks: Add reclaim plan kernel

## Phase 1: Model and proofs

- [x] [serial] I1 Add reachability, admission, validity, and soundness specifications. r[reclaim-plan.reachability] r[reclaim-plan.admission] r[reclaim-plan.validity]
- [x] [serial] I2 Prove `closed_set_contains_reachable` and `reachable_extend`. r[reclaim-plan.reachability]
- [x] [serial] I3 Prove `no_live_candidate`, `no_dangling_reference`, and `protected_never_candidate` from validity alone. r[reclaim-plan.no-live-candidate] r[reclaim-plan.no-dangling-reference] r[reclaim-plan.protected-closure]
- [x] [serial] I4 Add `check_plan` and `admissible` with `ensures` equal to their specifications. r[reclaim-plan.validity] r[reclaim-plan.admission]
- [x] [serial] I5 Add the fixpoint `plan` with a round cap and prove planned results valid and sound; prove `planned_result_reclaims_every_unreachable_node`. r[reclaim-plan.planner] r[reclaim-plan.no-leak]
- [x] [serial] I6 Prove with `count_true_set` and `count_true_bounded` that the planner never reaches its round cap, so admissible inputs always plan. r[reclaim-plan.planner]

## Phase 2: Tests

- [x] [parallel] V1 Add positive and negative tests, including corrupted plans. r[reclaim-plan.validity] r[reclaim-plan.admission]
- [x] [parallel] V2 Add an exhaustive test over every graph, root set, and reclaimable set with up to three nodes against an independent oracle. r[reclaim-plan.evidence]

## Phase 3: Evidence and lifecycle

- [x] [serial] I7 Add `docs/requirements.md` entries, the module export, and non-claims. r[reclaim-plan.evidence]
- [ ] [serial] I8 Add README rows and regenerate the module index and proof catalogs. r[reclaim-plan.evidence]
- [ ] [serial] V3 Run `./scripts/verus.sh src/lib.rs` with the pinned verifier, Tracey, and the proof-gap inventory; capture transcripts. r[reclaim-plan.evidence]
- [ ] [serial] V4 Run Cairn validation and the change gates; archive. r[reclaim-plan.evidence]
