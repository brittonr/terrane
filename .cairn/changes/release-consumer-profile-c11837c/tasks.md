## Phase 1: Pin and profile values

- [x] [serial] Point the `trellis-consumer-release-src` flake input at `git+https://github.com/OnixResearch/terrane.git?rev=c11837c5c6cdf2714fd9015da07196d53eea28eb`, re-lock only that input with `nix flake lock --update-input trellis-consumer-release-src`, and confirm with `git diff flake.lock` that no other input node changed. r[trellis.crate_packaging.consumer_release_profile_binding]
- [x] [serial] Measure every profile-bound path and `docs/proof-artifact-manifest.json` from revision `c11837c5c6cdf2714fd9015da07196d53eea28eb` with `git cat-file -s` and `git cat-file blob | b3sum`, then update `release_id`, `package.revision`, `package.dependency_coordinate`, every artifact row, and `verification.proof_manifest_bytes` / `proof_manifest_blake3`. r[trellis.crate_packaging.consumer_release_profile_binding]
- [x] [serial] Regenerate `consumer-profile/generated/profile.json` with `nickel export --format json consumer-profile/profile.ncl`, refresh the revision named in `docs/verified-logic-consumer-profile.md`, and add the `CHANGELOG.md` entry. r[trellis.crate_packaging.consumer_release_profile_binding]

## Phase 2: Validation

- [x] [serial] Run `./scripts/check-verified-logic-consumer-profile.rs` in the dev shell and capture the passing output plus its exit code. r[trellis.crate_packaging.consumer_release_profile_rebind_validation]
- [x] [serial] Run `nix build .#checks.x86_64-linux.consumer-release-profile --no-link -L` in the dev shell and capture the exit code, confirming the Nix gate observes the same revision as the profile. r[trellis.crate_packaging.consumer_release_profile_binding]
- [x] [serial] Run `./scripts/verus.sh src/lib.rs`, `tracey query uncovered`, and `tracey query untested`, and capture their exit codes and rule counts. r[trellis.crate_packaging.consumer_release_profile_rebind_validation]
- [ ] [serial] Run the full `./scripts/verify.rs` to exit 0 with every section green, and capture the transcript as validation evidence. r[trellis.crate_packaging.consumer_release_profile_rebind_validation]

## Phase 3: Lifecycle

- [ ] [serial] Sync the accepted delta spec, archive the change package, and validate the post-archive layout with `cairn validate --root . --policy cairn-policy/generated/cairn-policy.json --strict`. r[trellis.crate_packaging.consumer_release_profile_binding]
