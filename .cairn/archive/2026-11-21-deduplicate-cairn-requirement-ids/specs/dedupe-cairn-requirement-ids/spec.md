# Requirement ID Deduplication Specification

## Purpose

Make the accepted Cairn specification tree declare each requirement ID exactly
once, without changing any requirement's heading, body or scenarios.

## ADDED Requirements

### Requirement: Accepted requirement IDs are unique

r[dedupe-cairn-requirement-ids.unique]

Every requirement identifier line in `.cairn/specs` MUST be declared by exactly
one requirement block, so the content-bound requirement registry can export and
verify the repository.

#### Scenario: Registry export succeeds

- GIVEN the deduplicated accepted specs at a committed revision
- WHEN `cairn requirement-registry export` runs
- THEN it exits 0 with no duplicate-ID issue
