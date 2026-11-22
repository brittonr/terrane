## Why

The verified-logic consumer release profile still binds revision `df99309c31e95b8068d90153e362dad6234d0cca`, but commits after that revision changed three profile-bound files (`Cargo.toml`, `src/lib.rs`, `src/ucan_backend_transcript.rs`) and the proof artifact manifest. The standalone checker measures the worktree, while `checks.<system>.consumer-release-profile` measures the locked `trellis-consumer-release-src` input at its own revision and requires that revision to equal the profile revision. Both gates can only agree when the profile revision equals the worktree revision, so the profile is currently red in the first direction and would move the failure to the Nix gate if only the artifact rows were rewritten.

The recipe and the exact drift were recorded in `.cairn/archive/2026-11-22-repair-verify-gate-drift/evidence/consumer-profile-blocker.md`, which deferred the pin move. This change performs that deferred pin move against the published revision `c11837c5c6cdf2714fd9015da07196d53eea28eb`.

## What Changes

- Move the `trellis-consumer-release-src` flake input to `rev=c11837c5c6cdf2714fd9015da07196d53eea28eb` and re-lock only that input.
- Rebind `consumer-profile/profile.ncl` to the same revision: `release_id`, `package.revision`, `dependency_coordinate`, every artifact row, and the proof manifest byte count and BLAKE3 digest, all measured from the published tree at that revision.
- Regenerate `consumer-profile/generated/profile.json` from the Nickel source with `nickel export --format json`.
- Refresh the revision named in `docs/verified-logic-consumer-profile.md` and record the move in `CHANGELOG.md`.
- Add the delta requirement that the profile revision, the locked release source revision, and the measured worktree agree.

## Impact

- **Files**: `flake.nix`, `flake.lock`, `consumer-profile/profile.ncl`, `consumer-profile/generated/profile.json`, `docs/verified-logic-consumer-profile.md`, `CHANGELOG.md`, and this change package.
- **Testing**: `./scripts/check-verified-logic-consumer-profile.rs`, `nix build .#checks.x86_64-linux.consumer-release-profile`, `./scripts/verus.sh src/lib.rs`, `tracey query uncovered`, `tracey query untested`, and the full `./scripts/verify.rs`.
- **Boundary**: this is a source-identity and pin move only. It changes no verified code, no proof artifact content, no supported-claim text, and no non-claim. The profile keeps describing the same selected modules and the same proof boundary; only the immutable revision and the digests measured from it move.
- **Scope guard**: the commit changes no file the profile binds, so the worktree equals the bound revision for every artifact row.
