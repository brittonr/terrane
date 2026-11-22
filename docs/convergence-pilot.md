# Convergence pilot: bounds.clamp

The pilot turns one proved bounds requirement into reviewed producer evidence,
an Octet judgment and a Cairn convergence case.

## Producer job

`proof-rails/verus-bounds/mantle-job.ncl` is the Trellis owner contract
(`trellis-mantle-producer-job/v1`, owner `Trellis`, job
`trellis.bounds.verus.mantle.v1`). `nix/trellis-bounds-verus-job.nix` runs the
pinned Verus over all of `src/lib.rs`, fails closed on any non-passing result,
and emits `job.json`, `source/`, `source-revision`, `flake.lock`, the reviewed
specification, bounded `artifacts/`, `producer-report.json`,
`verus-receipt.json`, `formal-proof-envelope.json`, `proof-cards.json`,
`evidence.json`, `profile.json`, `provider-profile.json` and
`non-claims.json`. The claim is scoped to the bounds module; the checked run is
the whole crate, which the informal remainder states.

`tools/mantle-producer/export.py` freezes a committed revision's job graph
without executing it and prints `proof_executed: false`.

## Judgment

`tools/octet-judge/run.sh` runs Octet's `requirements check` and `verify` over a
realized producer root with `proof-rails/verus-bounds/contract.json`.
`proof-rails/verus-bounds/check-registry-negatives.sh` and
`tools/octet-judge/check-judge-negatives.sh` cover the registry, contract,
evidence, card and provider negatives.

## Convergence case

The case's evidence bundle carries the contract file's BLAKE3 in
`contract_blake3`, which Cairn requires the evidence to bind. The inspected
receipt is saved as `convergence/receipt.json`; the per-binding negative
records live in `convergence/negatives/` and run through
`check-convergence-negatives.py`.

`proof-rails/verus-bounds/convergence/case.json` binds the registry at the
producer source revision, the targets and generated source record, the Octet
contract, decision and evidence, the Mantle bridge handoff, the producer output
the bridge consumed, and the Mantle realization receipt and store-info record.
Inspect it with the pinned `cairn-convergence` binary:

```sh
cairn convergence inspect --case proof-rails/verus-bounds/convergence/case.json \
  --root <checkout at the registry revision>
```

## Realization

Mantle builds the frozen root with the `cache-inputs-build-roots-v1` route:
inputs are hydrated from the WP8-signed cache and the root is built by the
checked `mantle` executable from Mantle `5263dabfc…`
(`a3cf3924…`); the bridge is from a clean export of the same commit
(`604da752…`). The Mantle-built output
`/nix/store/5hqkrl54…-trellis-bounds-verus-job` is byte-identical to the
Nix-built output (same NAR, empty `diff -r`), and the convergence receipt
records route `cache-inputs-build-roots-v1`.

Mantle does not rebuild the hydrated inputs; their trust is the WP8 signing key
and the Nix cache. The bridge does not evaluate proofs. The refusal case is
recorded where Mantle's route checks that a cache offering the root is refused.

Octet's decision is the only acceptance. Cairn binds saved records; it does not
re-run any tool, authenticate producers or verify signatures.
