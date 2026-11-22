# Tasks

## Phase 1: Rebind records

- [x] [serial] Replace the realization receipt, store-info, `handoff-built`, and the realized producer-output copies with the E2 `cache-inputs-build-roots-v1` records and fill every `E2_*` placeholder from those records. r[trellis.convergence_pilot.built_root]
- [x] [serial] If E2 changed the producer source revision or the reviewed specification, update the registry/targets/contract/generated files and record why; otherwise leave them byte-identical. r[trellis.convergence_pilot.built_root]

## Phase 2: Judgment and negatives

- [x] [serial] Run `tools/octet-judge/run.sh` over the Mantle-built root, replace the decision and verification records, and record the decision identity. r[trellis.convergence_pilot.built_root]
- [x] [serial] Re-run `cairn convergence inspect` and save the new receipt; it MUST report `bounds.clamp` `current` and `targets_current: true`. r[trellis.convergence_pilot.built_root]
- [x] [parallel] Re-run the 20 per-binding negatives and confirm each expected state and issue code. r[trellis.convergence_pilot.built_root]

## Phase 3: Documentation and validation

- [x] [serial] Update `docs/convergence-pilot.md`, the CHANGELOG and the case README: drop both old non-claims and state the built-root trust boundary. r[trellis.convergence_pilot.built_root]
- [x] [serial] Run Tracey, `./scripts/verify.rs` with a fresh target on `/datapool` (documenting pre-existing base failures), the gates, sync and archive with `CAIRN_ARCHIVE_DATE`, and post-archive validate. r[trellis.convergence_pilot.built_root]
