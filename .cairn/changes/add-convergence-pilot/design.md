# Design: Convergence pilot producer job

## Context

The pilot target is `bounds.clamp`. `src/bounds.rs` proves `clamp` against
`clamp_spec` and proves the cross-requirement consequence
`in_range_spec(clamp_spec(value, lo, hi), lo, hi)` under `lo <= hi`. Terrane
does not prove the module standalone: `AGENTS.md` records that
`./scripts/verus.sh src/<module>.rs` treats the file as a crate root and cannot
resolve sibling `crate::` imports, so the checked proof runs `src/lib.rs` with
the same arguments as `checks.verus-proofs` and `scripts/verus.sh`.

There is no producer job, no consumer-shaped receipt, and no frozen tool graph
today. `docs/proof-evidence-bundle.json` is a local summary, not a verifier
receipt. The two bounds requirements live in `docs/requirements.md`, which the
content-bound registry cannot admit because it only accepts
`.cairn/specs/<name>/spec.md`.

## Decisions

### Decision: Move the bounds requirements into the Cairn spec tree (D-A)

**Choice:** move the `## Bounds` block, with its `bounds.in-range` and
`bounds.clamp` IDs and body text unchanged, to
`.cairn/specs/bounds/spec.md`; leave a pointer in `docs/requirements.md` and add
the new file to the Tracey `verified-logic` include.

**Rationale:** one normative copy, requirement IDs unchanged, all
`src/bounds.rs` markers still resolve. A hand-duplicated or generated
projection keeps two copies of normative text.

### Decision: Name the job with the `trellis-` family (D-F)

**Choice:** schema `trellis-mantle-producer-job/v1`, owner `Trellis`, job
`trellis.bounds.verus.mantle.v1`, profile `trellis-verus-producer-profile/v1`,
report `trellis-verus-producer-report/v1`.

**Rationale:** the existing verification bridge already labels this producer
`Trellis`; the job must not borrow Octet's schema or owner.

### Decision: Prove the whole crate and scope the claim to bounds (D-G)

**Choice:** run the pinned Verus release over `src/lib.rs` exactly as
`checks.verus-proofs` does, and scope the receipt, report, and claim to the
`bounds` module and the `trellis.bounds.verus#rail-0` rail.

**Rationale:** the checked run is the existing proof, so the pilot proves a
real theorem rather than a new standalone model. The checked scope is larger
than the claim, which the informal remainder states.

### Decision: Octet authors the envelope through a thin wrapper (D-H)

**Choice:** add a new flake input `octet-judge` pinned to an exact Octet main
revision, build a thin Rust binary that calls
`octet_standards::formal_proof::adapt_verus_v1_receipt`, and let the job write
the resulting `octet-formal-proof-envelope/v1` bytes. The existing `octet` and
`octet-aeneas` inputs stay byte-identical; the declaration/lock mismatch on the
existing `octet` input is left alone (D-I).

**Rationale:** the consumer stays the author of its own envelope mapping. The
wrapper is a translation, not a second acceptance policy: an unknown receipt
status returns an error and the job fails closed.

### Decision: Freeze is acquisition only (D-H, D-E boundary)

**Choice:** `tools/mantle-producer/export.py` reads `nix flake metadata`, the
exported job JSON, and the recursive derivation graph, then copies the exact
bytes. It never runs Verus and prints `proof_executed: false`.

**Rationale:** realization, observation, and judgment belong to Mantle, the
judge input, and Cairn. A preserve route, if used later, must be recorded as a
separate non-claim rather than presented as a build.

### Decision: New Cairn files remain MIT OR Apache-2.0 core material (D-J)

**Choice:** the new Terrane files authored by this change, including the
`.cairn/specs/bounds/spec.md` text, stay under the repository's existing
license policy; no upstream Cairn blob is vendored by this stage.

**Rationale:** the stage-1 files are owner-authored. Upstream Cairn vendoring
is a later stage that must record its own provenance.

## Producer job shape

`proof-rails/verus-bounds/mantle-job.ncl` is the owner contract:

- `schema_version = "trellis-mantle-producer-job/v1"`, `owner = "Trellis"`,
  `job_id = "trellis.bounds.verus.mantle.v1"`;
- `profile` and `provider` name the committed JSON identities;
- `source.root = "src"` with the exact bound paths and `network_allowed =
  false`;
- `execution.command_argv` is the pinned `terrane-verus` invocation,
  `execution.artifact_verify_argv` runs the Terrane checker, and
  `execution.rail_id = "trellis.bounds.verus#rail-0"`;
- `limits` fix the output bounds and core budget; `non_claims` state what the
  job does not claim.

The generated `profile.json` and `provider-profile.json` are Nickel exports
checked for freshness by the derivation, mirroring how the consumer job checks
its committed profile.

## Derivation and outputs

`nix/trellis-bounds-verus-job.nix` is a `runCommand` with the pinned Verus
release, the pinned Rust toolchain, the filtered `src/` tree, Nickel, jq, and
the checker binaries. It:

1. copies the filtered source to `$out/source/` and runs Verus only on
   `$out/source/src/lib.rs`;
2. exports `job.json` from the Nickel contract and fails if the committed
   profile or provider profile is stale;
3. captures bounded stdout and stderr and fails closed unless the process exits
   0 and the writer observes the configured pass signal;
4. writes `verus-receipt.json`, `producer-report.json`, `proof-cards.json`,
   `evidence.json`, `profile.json`, `provider-profile.json`, and
   `non-claims.json` from the observed bytes, never reconstructing a receipt
   from an exit status;
5. writes the envelope from the Octet adapter, then runs the checker's
   `verify` mode against the whole output.

Outputs are the design contract: `job.json`, `source/`, `source-revision`,
`flake.lock`, `artifacts/verus.stdout`, `artifacts/verus.stderr`,
`artifacts/verus.command`, `producer-report.json`, `verus-receipt.json`,
`formal-proof-envelope.json`, `proof-cards.json`, `evidence.json`,
`profile.json`, `provider-profile.json`, `non-claims.json`.

## Freeze

`export.py` verifies the flake metadata revision equals `--revision`, requires
the owner files, exports the job JSON, evaluates the package `drvPath`, calls
`nix derivation show --recursive`, validates the v4 or legacy graph with the
consumer's bounds and node-shape rules, and copies `job.json`,
`tool-closure.derivation-json.json`, `flake-metadata.json`, `flake.lock`,
`root-derivation`, `source-revision`, and `source-payloads/`.

## Risks / Trade-offs

- The judge input is a second Octet revision. The provider pin and the judge
  revision are recorded separately; the judgment is only meaningful for the
  judge revision.
- The checked Verus run covers the whole crate while the claim covers
  `bounds`; the informal remainder and non-claims state the difference.
- A passing receipt is recorded evidence, not theorem admission: Mantle and
  Cairn still perform their own bindings in later stages.
