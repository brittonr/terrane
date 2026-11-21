# Convergence Pilot Specification

## Purpose

Define the Trellis-owned producer job for one proved bounds requirement, its
exact-byte outputs, and the acquisition-only freeze that exposes its tool
graph. The pilot reports evidence; it does not judge its own theorem or
realize its own build.

## ADDED Requirements

### Requirement: Bounds requirements have one reviewed specification

r[trellis.convergence_pilot.reviewed_spec]

Trellis MUST keep the `bounds.in-range` and `bounds.clamp` requirement IDs and
body text unchanged in `.cairn/specs/bounds/spec.md`, MUST leave only a pointer
to that file in `docs/requirements.md`, and MUST keep Tracey coverage complete
over the new location.

#### Scenario: Moved requirement keeps its identity

- GIVEN the bounds requirement text has moved to `.cairn/specs/bounds/spec.md`
- WHEN Tracey queries `uncovered` and `untested`
- THEN the counts MUST be no worse than before the move and the `src/bounds.rs` markers MUST still resolve

#### Scenario: Legacy document does not duplicate normative text

- GIVEN a reader opens the `## Bounds` section of `docs/requirements.md`
- WHEN they follow its pointer
- THEN the normative requirement text MUST live only in `.cairn/specs/bounds/spec.md`

### Requirement: The producer job is owner-authored and bounded

r[trellis.convergence_pilot.producer_job]

Trellis MUST author one producer job contract with schema
`trellis-mantle-producer-job/v1`, owner `Trellis`, job
`trellis.bounds.verus.mantle.v1`, the pinned Verus invocation over `src/lib.rs`,
`network_allowed = false`, bounded output limits, and an explicit non-claim
list. The job MUST name the committed profile and provider-profile identities.

#### Scenario: Contract admits the Trellis owner

- GIVEN the committed job contract
- WHEN its Nickel source is exported
- THEN the schema, owner, job ID, and rail ID MUST be the Trellis values and no Octet identity MAY appear

#### Scenario: Network is denied

- GIVEN the job contract is evaluated
- WHEN either source or execution network policy is read
- THEN both MUST be `false`

### Requirement: The job output binds the observed proof bytes

r[trellis.convergence_pilot.job_output]

The job MUST emit `job.json`, `source/`, `source-revision`, `flake.lock`,
bounded `artifacts/`, `producer-report.json`
(`trellis-verus-producer-report/v1`), `verus-receipt.json`
(`octet-verus-proof-receipt/v1`), `proof-cards.json` (`octet-proof-cards/v1`),
`evidence.json` (`octet-requirement-evidence/v1`), `profile.json`
(`trellis-verus-producer-profile/v1`), `provider-profile.json`, and
`non-claims.json`. A receipt MUST be written only from observed bytes and the
configured pass signal, and any non-passing verifier result MUST fail the
build.

#### Scenario: Passing run emits bound evidence

- GIVEN the pinned Verus run exits 0 with the configured pass signal
- WHEN the writer runs
- THEN the receipt, report, cards, evidence, and profile MUST name the same source, artifact, and profile identities

#### Scenario: Failed run emits no passing evidence

- GIVEN the verifier exits non-zero or the pass signal is absent
- WHEN the job runs
- THEN the derivation MUST fail and no passing receipt MAY be admitted

#### Scenario: Tampered artifact fails verification

- GIVEN a written output whose artifact bytes changed after the receipt was written
- WHEN the checker's `verify` mode runs
- THEN it MUST reject the output with a non-zero exit

### Requirement: Octet authors the formal proof envelope

r[trellis.convergence_pilot.envelope]

The job MUST write `octet-formal-proof-envelope/v1` with provider
`verus-v1-compatibility` by calling Octet's `adapt_verus_v1_receipt` through a
thin wrapper built against the separately pinned `octet-judge` input. The
existing `octet` and `octet-aeneas` inputs MUST NOT move.

#### Scenario: Known status is adapted

- GIVEN a receipt with a known status
- WHEN the wrapper runs
- THEN the envelope MUST preserve the receipt bytes and name the Verus provider

#### Scenario: Unknown status fails closed

- GIVEN a receipt with an unknown status spelling
- WHEN the wrapper runs
- THEN it MUST exit non-zero and write no envelope

### Requirement: Freeze is acquisition only

r[trellis.convergence_pilot.freeze]

`tools/mantle-producer/export.py` MUST verify the immutable flake revision,
export the job JSON, evaluate the package derivation, read the recursive
derivation graph in the requested raw format, and copy `job.json`,
`tool-closure.derivation-json.json`, `flake-metadata.json`, `flake.lock`,
`root-derivation`, `source-revision`, and `source-payloads/`. It MUST NOT run
the verifier, import a recorded receipt, or rewrite the graph into another
format, and it MUST print `proof_executed: false`.

#### Scenario: Committed revision freezes

- GIVEN a committed revision with the owner files
- WHEN the exporter runs against that immutable flake reference
- THEN the output MUST contain the listed files and the summary MUST report `proof_executed: false`

#### Scenario: Wrong revision or malformed graph is rejected

- GIVEN a revision that differs from the flake metadata, a graph without its root, an incomplete closure, or an unsupported node schema
- WHEN the exporter validates it
- THEN it MUST fail with a diagnostic and write no output

### Requirement: Positive and negative fixtures cover the new surfaces

r[trellis.convergence_pilot.fixtures]

Each new checker MUST have positive and negative fixtures, and the repository
checks touched by the change MUST pass or be recorded with their exact
diagnostic.

#### Scenario: Negative fixtures fail closed

- GIVEN a stale profile, a failed verifier, a tampered artifact, an unknown status, a wrong revision, or a malformed graph
- WHEN the relevant checker runs
- THEN it MUST exit non-zero with a diagnostic naming the rejected condition
