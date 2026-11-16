# Reclaim Plan Delta

## ADDED Requirements

### Requirement: Removal never touches a live node

r[reclaim-plan.no-live-candidate] No candidate of a valid plan MAY be reachable from any root, where reachability is r[reclaim-plan.reachability].

#### Scenario: Corrupted retention

- **GIVEN** a plan that drops a node a root reaches from retention
- **WHEN** the plan is checked
- **THEN** `check_plan` MUST reject it

### Requirement: Removal leaves no dangling references

r[reclaim-plan.no-dangling-reference] After removing every candidate of a valid plan, no surviving node MAY reference a removed node.

#### Scenario: Protected node references a reclaimable node

- **GIVEN** a protected node that references a reclaimable node
- **WHEN** planning runs
- **THEN** it MUST reject with the protected-closure class, per r[reclaim-plan.protected-closure]

### Requirement: Planning is valid and leak-free

r[reclaim-plan.planner] A planned result MUST be valid per r[reclaim-plan.validity], and inadmissible inputs MUST reject per r[reclaim-plan.admission]. r[reclaim-plan.no-leak] Every retained node MUST be reachable from a root.

#### Scenario: Unreachable reclaimable node

- **GIVEN** a reclaimable node no root reaches
- **WHEN** planning succeeds
- **THEN** the node MUST be a candidate

### Requirement: Optimal, deterministic, monotone planning

r[reclaim-plan.exact-retention] A planned result MUST retain exactly the live nodes. r[reclaim-plan.least-plan] It MUST be the least valid plan for its input, and any two planned results for the same input MUST be equal. r[reclaim-plan.monotone] Adding roots or references, or protecting more nodes, MUST NOT add a candidate. r[reclaim-plan.set-invariance] Reordering or duplicating roots or references within the bounds MUST NOT change validity or soundness.

#### Scenario: Root added between planning and execution

- **GIVEN** a planned result for some roots
- **WHEN** a root is added and any valid plan is made for the new input
- **THEN** every candidate of the new plan MUST be a candidate of the original planned result

#### Scenario: Consumer deduplicates references

- **GIVEN** a planned result for some roots and references
- **WHEN** a consumer sorts and deduplicates both before checking
- **THEN** the same retained and candidate sets MUST remain valid and sound

### Requirement: Validated key projection

r[reclaim-plan.key-projection] A consumer's key-to-index mapping and plan path lists MUST be validated against the keys' bytes, with unsorted or duplicate keys and any unresolved or mismatched index refused, and the safety theorems MUST then hold over keys.

#### Scenario: Consumer maps a reference to the wrong entry

- **GIVEN** a reference index that names an entry whose key differs from the referenced bytes
- **WHEN** the projection is validated
- **THEN** it MUST be refused

### Requirement: Evidence and non-claims

r[reclaim-plan.evidence] Each property MUST have a named proof function and exhaustive small-graph tests, with explicit non-claims for root and reference completeness, path mapping, plan identity, byte totals, and removal execution.

#### Scenario: Stronger claim requested

- **GIVEN** all proofs pass
- **WHEN** evidence claims the store's roots were complete or the removals executed correctly
- **THEN** the claim MUST be rejected as outside the proof boundary
