# Convergence Pilot Specification (built-root rebinding)

## ADDED Requirements

### Requirement: The case binds the Mantle-built root
r[trellis.convergence_pilot.built_root]

The convergence case MUST bind the producer root that Mantle built with the
`cache-inputs-build-roots-v1` route: its realization receipt, store-info
record, bridge handoff and realized output copies MUST be the E2 bytes, and the
inspect receipt MUST report `bounds.clamp` `current` with
`targets_current: true`. The case documentation MUST NOT retain the
`cache-only-preserve-v1` or unchecked-executable non-claims.

#### Scenario: Inspect reports current

- GIVEN the E2 records and the Mantle-built root
- WHEN `cairn convergence inspect` runs with the case and the registry revision
- THEN it prints one receipt with `bounds.clamp` `current` and `targets_current: true`

#### Scenario: Old route non-claims are gone

- GIVEN the case README and the pilot documentation
- WHEN they are reviewed
- THEN no preserve-route or unchecked-executable non-claim remains
