## Context

`consumer-profile/profile.ncl` binds the published Terrane source at revision `df99309c31e95b8068d90153e362dad6234d0cca` and records one artifact row per selected path plus the proof artifact manifest identity. `scripts/check-verified-logic-consumer-profile.rs` exports the Nickel profile, measures `--source-root` (default: the worktree), and compares the measurements against the rows. The Nix check `checks.<system>.consumer-release-profile` copies `${trellis-consumer-release-src}` into the store and passes `--source-root ${trellis-consumer-release-src}` with `--source-revision ${trellis-consumer-release-src.rev}`; `validate_observed_source_revision` then requires the observed revision to equal the profile revision.

Commit `96c5282` rewrote `Cargo.toml`, `src/lib.rs`, `src/ucan_backend_transcript.rs`, and `docs/proof-artifact-manifest.json` after `df99309`. The current worktree therefore differs from the record in exactly four measured identities: three artifact rows and the proof manifest. `main` at `c11837c5c6cdf2714fd9015da07196d53eea28eb` is already published, so the profile and the flake pin can move together onto it.

## Decisions

### Decision: Move the pin to the published `main` revision `c11837c`

**Choice:** Set `inputs.trellis-consumer-release-src` to `git+https://github.com/OnixResearch/terrane.git?rev=c11837c5c6cdf2714fd9015da07196d53eea28eb` and the profile revision to the same commit, then re-lock with `nix flake lock --update-input trellis-consumer-release-src`.

**Rationale:** The two gates measure different trees and compare both against the profile revision, so a pin move and a profile rebind are one atomic change. `c11837c` is the merge commit of the verify-gate repair already on the default branch, so the pinned input resolves without any new publication step, and the worktree of this branch is that tree plus only unbound files.

### Decision: Measure bound identities from the published tree, not the working copy

**Choice:** Read every bound path with `git cat-file blob c11837c:<path>` and hash it with `b3sum`, and take byte counts from `git cat-file -s`.

**Rationale:** The record must describe the immutable revision, which is what the Nix check measures. Hashing the working copy would only be correct while the working copy happens to be identical, and it hides accidental edits to bound files.

### Decision: Regenerate the JSON projection with the documented exporter

**Choice:** Produce `consumer-profile/generated/profile.json` with `nickel export --format json consumer-profile/profile.ncl`; the checker compares the parsed projection against the exported Nickel.

**Rationale:** The projection is a generated artifact. The documented refresh route in `docs/verified-logic-consumer-profile.md` is `nickel export`, and the existing projection is byte-identical to that command's output for the current source, so the same command keeps the artifact reproducible.

### Decision: Keep the artifact set and claim text fixed

**Choice:** Change only revision, coordinate, release ID suffix, artifact bytes/digests, and the proof manifest identity. Keep the selected module list, supported features, supported claim, and required non-claims unchanged.

**Rationale:** The move corrects source identity drift; it introduces no new selected surface and no new verification claim. Leaving the claim text untouched keeps the boundary statements reviewed earlier intact.

## Risks / Trade-offs

- Moving the lock rewrites a generated file. Mitigation: lock only through `nix flake lock --update-input trellis-consumer-release-src` and confirm with `git diff flake.lock` that no other input node changed.
- Any later commit that touches a bound path re-opens the same drift. Mitigation: the added requirement records that the worktree and the locked source must agree with the profile revision, so the next such commit is expected to move the pin in the same change.
- The revision names a merge commit rather than a dedicated release tag. Mitigation: the pin, the profile revision, and `docs/verified-logic-consumer-profile.md` all name the identical immutable SHA, and the Nix check resolves that SHA directly.
