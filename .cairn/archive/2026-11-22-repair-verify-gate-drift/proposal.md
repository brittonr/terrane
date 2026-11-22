## Why

`./scripts/verify.rs` fails five recorded-evidence checks at `1da01e2`, and the
same five reproduce at `b9bd540`. The failures are not proof or code failures:
each one is a recorded artifact that was left behind when its input changed, or a
gate that needs an input the local checkout did not have.

Commit `96c5282` ("fix: support October Verus and Sail") changed
`src/scheme_logic/syntax.rs`, `docs/proof-evidence-bundle.json`, `Cargo.toml`,
`src/lib.rs`, and `src/ucan_backend_transcript.rs` without re-running the exporters
and refresh procedures those files feed. The Aspen evidence index additionally
requires a sibling `../aspen` checkout that the failing environment did not have.

## What Changes

- Refresh the `trellis.scheme.reader.structure` source identity
  (`docs/proof-artifact-manifest.json`) and re-bind the Preserves envelope digest
  that carries the manifest identity. r[verification-tooling.recorded-evidence-refresh]
- Regenerate the standalone Nickel export exact-byte destination through the
  owning exporter (`--write`) so the dual-run comparison passes again.
  r[verification-tooling.recorded-evidence-refresh]
- Regenerate the Aeneas cross-verifier receipt and manifest (`--write`) so the
  receipt rebinds the refreshed Verus receipt material.
  r[verification-tooling.recorded-evidence-refresh]
- Validate every Aspen evidence-index row against a sibling `../aspen` checkout
  and record the checkout prerequisite in this change's evidence.
  r[verification-tooling.recorded-evidence-refresh]
- Record the consumer release-profile drift and its exact pin blocker without
  moving any pin or editing `flake.lock`.
  r[verification-tooling.recorded-evidence-refresh]

## Impact

- **Files**: `docs/proof-artifact-manifest.json`,
  `docs/proof-artifact-manifest.preserves-envelope.json`,
  `nickel-export-cutover/generated/outputs/proof-evidence-contract.json`,
  `aeneas-pilot/generated/aeneas-proof-receipt.json`,
  `aeneas-pilot/generated/cross-verifier-manifest.json`, and this change package.
- **Pins**: none move. `octet-public@bf633b7`, `cairn-public@acb4c3d`,
  `cairn-convergence`, and `flake.lock` stay byte-identical.
- **Proof claims**: no source, spec, proof, or trusted-boundary change. The repair
  restores recorded-identity freshness only; it proves nothing about Aeneas, Charon,
  Lean, Nickel, or downstream consumers.
- **Not fixed**: the verified-logic consumer release profile can only be advanced
  by moving `inputs.trellis-consumer-release-src`, which this change must not do.
