# Consumer Release Profile Specification

## Purpose

Defines the `consumer-release-profile` capability.

## Requirements

### Requirement: Verified-logic consumer profile release binding
r[trellis.crate_packaging.consumer_release_profile_binding] Trellis MUST bind the verified-logic consumer profile, the locked `trellis-consumer-release-src` flake input, and the measured worktree to one identical immutable published revision, and every artifact row in the profile MUST record the byte count and BLAKE3 digest measured from that revision.

#### Scenario: Profile, pin, and worktree agree

- GIVEN `consumer-profile/profile.ncl` names revision `R` and records artifact rows measured from `R`
- WHEN `./scripts/check-verified-logic-consumer-profile.rs` measures the worktree and `checks.<system>.consumer-release-profile` measures `${trellis-consumer-release-src}`
- THEN both gates MUST pass, and the pin locked in `flake.lock` MUST resolve to `R`

#### Scenario: A bound file moves without the pin

- GIVEN a commit changes a profile-bound path after the recorded revision
- WHEN the profile checker measures that worktree
- THEN it MUST fail with drift diagnostics naming each changed artifact instead of reporting alignment

#### Scenario: Only the release source input is re-locked

- GIVEN the release source input moves to a new revision
- WHEN the lock is refreshed with `nix flake lock --update-input trellis-consumer-release-src`
- THEN the `flake.lock` diff MUST change only that input's node and its revision

### Requirement: Consumer profile rebind validation
r[trellis.crate_packaging.consumer_release_profile_rebind_validation] Trellis MUST validate a consumer profile rebind with both profile gates, the maintained Verus proof run, the Tracey coverage queries, and the full repository verification rail before the change is synced and archived.

#### Scenario: Rebind is validated on both measured trees

- GIVEN the profile and the locked release source pin name the same revision
- WHEN the standalone profile checker and `checks.<system>.consumer-release-profile` run
- THEN both MUST exit successfully, and the proof run, both Tracey queries, and `./scripts/verify.rs` MUST also succeed for the rebind commit

#### Scenario: Drift keeps the change unarchived

- GIVEN any profile gate, proof run, Tracey query, or `./scripts/verify.rs` run fails
- WHEN the change is prepared for sync or archive
- THEN the change MUST remain unarchived until the failing check passes
