# Tasks

## Phase 1: Reviewed specification

- [x] [serial] Record Tracey `uncovered` and `untested` before the move, then add `.cairn/specs/bounds/spec.md` with the unchanged `bounds.in-range` and `bounds.clamp` text, replace the `docs/requirements.md` block with a pointer, and add the spec to the Tracey include. r[trellis.convergence_pilot.reviewed_spec]
- [x] [serial] Rerun Tracey `uncovered` and `untested` and record that the requirement IDs and coverage counts are unchanged. r[trellis.convergence_pilot.reviewed_spec]

## Phase 2: Producer job contract and derivation

- [x] [serial] Add `proof-rails/verus-bounds/mantle-job.ncl` with the Trellis owner, bounded command, denied network, and non-claims, plus the generated profile and provider-profile Nickel sources and JSON. r[trellis.convergence_pilot.producer_job]
- [x] [serial] Add `nix/trellis-bounds-verus-job.nix` and the `trellis-bounds-verus-job` flake package that runs the pinned Verus over `src/lib.rs` and fails closed on any non-passing result. r[trellis.convergence_pilot.job_output]
- [x] [parallel] Add negative checks for a stale generated profile, a non-zero Verus exit, and a missing artifact. r[trellis.convergence_pilot.fixtures]

## Phase 3: Receipt, envelope, and evidence writer

- [x] [serial] Add the `trellis-verus-producer` checker with `emit` and `verify` modes that write the receipt, report, proof card deck, evidence bundle, profile, provider profile, and non-claims from observed bytes. r[trellis.convergence_pilot.job_output]
- [x] [serial] Add the `octet-judge` flake input at the recorded revision and a thin Rust wrapper that writes `formal-proof-envelope.json` through `adapt_verus_v1_receipt`. r[trellis.convergence_pilot.envelope]
- [x] [parallel] Add positive and negative writer fixtures: passing output, failed exit, tampered stdout digest, unknown receipt status, and wrong provider identity. r[trellis.convergence_pilot.fixtures]
- [x] [parallel] Add envelope wrapper tests for a passing receipt, a failed receipt, and an unknown status. r[trellis.convergence_pilot.fixtures]

## Phase 4: Freeze and validation

- [x] [serial] Add `tools/mantle-producer/export.py` with acquisition-only freeze output and validate the v4 and legacy graph shapes. r[trellis.convergence_pilot.freeze]
- [x] [parallel] Add exporter negative fixtures for a wrong revision, a missing root, an incomplete closure, an unsupported schema version, and a source bound. r[trellis.convergence_pilot.fixtures]
- [x] [serial] Build `trellis-bounds-verus-job`, check the output file list, and freeze a committed revision into the convergence evidence directory. r[trellis.convergence_pilot.freeze]
- [ ] [serial] Run `./scripts/verify.rs`, Tracey uncovered and untested, the focused producer tests, and Cairn validation and gates; record any blocked check with its exact diagnostic. r[trellis.convergence_pilot.fixtures]

## Phase 5: Later stages (not this change's stage-1 commit set)

- [ ] [serial] Vendor the Cairn assurance-targets contract and policy with recorded provenance. r[trellis.convergence_pilot.freeze]
- [ ] [serial] Add the second Cairn input, registry export and targets, Octet judgment, Mantle realization, and the convergence case. r[trellis.convergence_pilot.producer_job]
