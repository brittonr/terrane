# bounds Specification

## Purpose

Defines the verified inclusive-range predicate and clamp operation, including
the cross-requirement guarantee that a clamped value is always in range. The
proofs run with the pinned Verus release over `src/lib.rs` and the exec
contracts live in `src/bounds.rs`.

## Requirements

### Requirement: In-range predicate
r[bounds.in-range]

The in-range predicate MUST return true if and only if the value is
within the inclusive range [lo, hi].

#### Scenario: Endpoints are in range

- GIVEN an inclusive range with `lo <= hi`
- WHEN the predicate is asked about `lo` or `hi`
- THEN it MUST return true

#### Scenario: Value outside the range is rejected

- GIVEN a value strictly below `lo` or strictly above `hi`
- WHEN the predicate is evaluated
- THEN it MUST return false

#### Scenario: Empty range contains nothing

- GIVEN `lo > hi`
- WHEN the predicate is evaluated for any value
- THEN it MUST return false

### Requirement: Clamp
r[bounds.clamp]

Clamp MUST return lo when value < lo, hi when value > hi, and value
otherwise. The result MUST always satisfy in-range(result, lo, hi).

#### Scenario: Below-range value clamps to lo

- GIVEN `value < lo` and `lo <= hi`
- WHEN clamp runs
- THEN it MUST return `lo`

#### Scenario: Above-range value clamps to hi

- GIVEN `value > hi` and `lo <= hi`
- WHEN clamp runs
- THEN it MUST return `hi`

#### Scenario: In-range value is unchanged

- GIVEN `lo <= value <= hi`
- WHEN clamp runs
- THEN it MUST return `value`

#### Scenario: Result is always in range

- GIVEN `lo <= hi`
- WHEN clamp returns for any value
- THEN the result MUST satisfy the in-range predicate for the same bounds
