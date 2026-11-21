# Proposal: Convergence pilot for one proved bounds requirement

## Why

Trellis proves `bounds.clamp` with Verus, but the proof is only a local
whole-crate check (`flake.nix` `checks.verus-proofs`). There is no producer
job, no verifier receipt in a consumer's shape, no maintainer-owned evidence
bundle, and no frozen tool graph another system can consume. The two bounds
requirements are also prose in the legacy Tracey document rather than a
reviewed Cairn specification, so a content-bound registry row cannot cite them.

The convergence pilot turns the existing proof into a named Trellis producer
job: an owner-authored job contract, a sandboxed derivation that runs the
pinned Verus release over `src/lib.rs`, a receipt and envelope authored in the
consumer's schema, and an acquisition-only export of the exact tool graph.
Downstream systems can then realize, observe, and judge the output without
Trellis re-implementing their acceptance policy.

## What Changes

- Move the `bounds.in-range` and `bounds.clamp` requirement text from
  `docs/requirements.md` into `.cairn/specs/bounds/spec.md` and point Tracey at
  the new location, keeping the requirement IDs and body text unchanged.
  r[trellis.convergence_pilot.reviewed_spec]
- Add the Trellis-owned producer job contract for one Verus run over
  `src/lib.rs`, with a bounds-scoped claim, a denied network, and fail-closed
  status handling. r[trellis.convergence_pilot.producer_job]
- Add a Nix package that derives the job output: exact source copy, job JSON,
  bounded verifier streams, receipt, envelope, proof card deck, evidence
  bundle, profile, provider profile, non-claims, source revision and lock.
  r[trellis.convergence_pilot.job_output]
- Author `formal-proof-envelope.json` by calling the consumer's
  `adapt_verus_v1_receipt` through a thin wrapper built against a separate
  pinned judge input, leaving the existing Octet pins untouched.
  r[trellis.convergence_pilot.envelope]
- Add an acquisition-only exporter that freezes a committed revision's job
  graph without executing the proof. r[trellis.convergence_pilot.freeze]
- Add positive and negative fixtures for the job contract, the writer, the
  envelope wrapper, and the exporter. r[trellis.convergence_pilot.fixtures]

## Impact

- **Files**: `.cairn/specs/bounds/spec.md`, `docs/requirements.md`,
  `.config/tracey/config.styx`, `proof-rails/verus-bounds/`,
  `nix/trellis-bounds-verus-job.nix`, `flake.nix`, `tools/verus-producer/`,
  `tools/octet-envelope-wrapper/`, `tools/mantle-producer/export.py`.
- **Testing**: the bounds spec move is checked with Tracey coverage before and
  after; the job is built with `nix build .#trellis-bounds-verus-job`; the
  writer and exporter run their fixture self-tests; the repository checks are
  `./scripts/verify.rs`, `tracey query uncovered`, and `tracey query untested`.
- **Boundary**: no existing Octet or Cairn pin moves, and no acceptance policy
  is reimplemented. The job reports a Trellis-owned result; it does not judge
  its own theorem, realize its own output, or claim release eligibility.
