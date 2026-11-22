# Verification Tooling Delta

## ADDED Requirements

### Requirement: Recorded Evidence Refresh
r[verification-tooling.recorded-evidence-refresh]

A change that alters an input of a recorded evidence artifact MUST refresh that
artifact through its owning exporter or documented refresh procedure before the
repository gate may claim green. Refresh MUST NOT relax a comparison, exempt a
check, hand-edit generated exporter output, move a flake pin, or edit
`flake.lock`; a refresh that would require any of those MUST be reported as a
blocker with the exact pin it needs.

#### Scenario: Exporter-owned artifact is regenerated

- GIVEN a checker byte-compares or digest-binds output produced by an existing exporter or generator
- WHEN the refreshed artifact is produced by that tool's write mode and the check mode re-run
- THEN the check MUST pass without any recorded value being written by hand

#### Scenario: Carrier digest follows a refreshed source manifest

- GIVEN a carrier records the digest of a source manifest whose row identity was refreshed
- WHEN the freshness check runs
- THEN the carrier digest MUST match the refreshed manifest and MUST fail closed when it does not

#### Scenario: External checkout prerequisite is satisfied, not bypassed

- GIVEN a checker validates recorded commits and blobs of another repository checkout
- WHEN the sibling checkout is present at the documented relative path
- THEN every recorded row MUST validate against it
- AND the checker MUST NOT be relaxed to skip source validation

#### Scenario: Pin-bound artifact is reported instead of advanced

- GIVEN a recorded artifact can only be refreshed by moving a pinned source revision
- WHEN the pin must not move in this change
- THEN the change MUST record the failing check, its root cause, and the exact pin it would need
- AND the artifact MUST NOT be rewritten against a mismatched revision
