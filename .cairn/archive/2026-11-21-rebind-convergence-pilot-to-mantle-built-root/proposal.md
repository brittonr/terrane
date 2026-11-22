# Proposal: Rebind the convergence pilot to Mantle's built root

## Why

The archived convergence pilot bound a producer output that Nix built and
Mantle preserved through `cache-only-preserve-v1`. Two non-claims followed from
that route: Mantle did not build the root, and the Mantle and bridge
executables were not the final checked ones. Mantle now has a real build route,
`cache-inputs-build-roots-v1`, landed at `f7c7fd09…` and checked with a
clean-export build. The convergence case must bind the root Mantle actually
built with those executables.

## What Changes

- Replace the case's realization receipt, store-info, handoff and realized
  producer-output copies with the `cache-inputs-build-roots-v1` records and the
  Mantle-built root. r[trellis.convergence_pilot.built_root]
- Re-run the final Octet judgment over the Mantle-built root and replace the
  decision and verification records. r[trellis.convergence_pilot.built_root]
- Re-run the 20 per-binding convergence negatives. r[trellis.convergence_pilot.built_root]
- Drop both old non-claims (preserve route, unchecked executables) from the
  docs, case README and evidence, and state the new facts plainly.
  r[trellis.convergence_pilot.built_root]

## Unchanged

The requirement registry, targets, contract and generated source record stay
byte-identical: the producer source revision and the reviewed specification do
not change. If E2 forces any of them to change, the change records why.

## Impact

- **Files**: `proof-rails/verus-bounds/convergence/` records and README,
  `docs/convergence-pilot.md`, `.cairn` change package.
- **Testing**: `cairn convergence inspect` must report `bounds.clamp` `current`
  with `targets_current: true`; the 20 negatives must behave as specified;
  Tracey and `verify.rs` with a fresh target.
- **Boundary**: E2's facts (paths, hashes, receipt fields) are filled in when
  the Mantle build route finishes. This change does not modify Cairn, Mantle,
  Octet or the producer job.
