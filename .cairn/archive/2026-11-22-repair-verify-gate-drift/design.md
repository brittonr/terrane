## Context

Five `./scripts/verify.rs` sections fail on a clean checkout of `1da01e2`. Each
checker compares a checked-in recorded artifact against bytes it measures or
re-exports from the current tree:

- `scripts/check-aspen-evidence-index.rs` resolves `../aspen` and re-reads
  recorded Aspen commits and blobs.
- `scripts/check-nickel-export-cutover.rs` re-runs the pinned `nickel-export`
  shell and compares its exact bytes with `nickel-export-cutover/generated/outputs/`.
- `scripts/check-aeneas-cross-verifier.rs` rebuilds a receipt binding the
  profile, source, artifacts, and the Verus receipt material.
- `scripts/check-verified-primitive-manifest.rs` measures source and proof bytes
  against `docs/proof-artifact-manifest.json`.
- `scripts/check-verified-logic-consumer-profile.rs` measures the release
  profile's artifact rows against a source root.

## Decisions

### 1. Refresh through the owning generator, never by hand-editing generated output

The cutover destination and the Aeneas receipt/manifest are rewritten only by
their `--write` modes, which re-export through the pinned tools and then re-run
`--check`. The manifest row and its Preserves carrier digest have no generator;
`docs/proof-artifact-manifest.md` names "refresh the row identity" as the owning
procedure, so the recorded BLAKE3/byte values are recomputed from the measured
file and the carrier digest is re-bound to the resulting manifest digest.

### 2. No gate is weakened and no pin moves

The repair changes recorded identities only. It does not relax a comparison, add
an exemption, skip a check, edit `flake.lock`, or move any flake input.

### 3. The Aspen index prerequisite is a checkout, not a code change

`docs/aspen-proof-snapshots.md` already declares the Aspen checkout the source of
truth for index rows. The failure was a missing sibling checkout, so the repair
supplies it and validates all eight rows; no script or index content changes.

### 4. The consumer release profile is blocked on a pin move and stays reported

`consumer-profile/profile.ncl` binds revision `df99309` and its rows match that
tree exactly. `scripts/check-verified-logic-consumer-profile.rs` measures the
worktree, while `flake.nix` `consumer-release-profile` measures
`inputs.trellis-consumer-release-src` pinned at `df99309` and asserts the observed
revision equals the profile revision. Both can only pass together if the profile
revision equals the worktree revision, which requires moving that input and
updating `flake.lock`. Rewriting only the profile rows would move the failure to
the Nix gate and would misdescribe the pinned revision, so this change records the
blocker and leaves the profile untouched.

### 5. Local rail environment

The pinned remote builder `aspen1.local` cannot satisfy its substituter, so the
local rail runs with remote builders disabled and `min-free = 0`; the fenix
`minimal.toolchain` the checker shebangs require is built into the local store
first. This is an environment prerequisite, not a repository change.

## Risks / Trade-offs

- Refreshing recorded identities advances the recorded evidence, so review must
  confirm the underlying inputs really changed for the reasons above rather than
  masking an unexplained diff.
- While the consumer profile stays on `df99309`, its
  `verification.proof_manifest_{bytes,blake3}` no longer match the refreshed
  manifest; that row and the release revision must be advanced together by the
  pin-moving release change.

## Non-Goals

- No source, proof, spec, or requirement change; no new Verus obligation.
- No Aeneas, Charon, Lean, or Nickel re-execution claim beyond the pinned exporter
  and receipt bindings the checkers themselves perform.
- No consumer release revision advance.
