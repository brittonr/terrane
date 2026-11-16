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

### Requirement: Evidence and non-claims

r[reclaim-plan.evidence] Each property MUST have a named proof function and exhaustive small-graph tests, with explicit non-claims for root and reference completeness, path mapping, plan identity, byte totals, and removal execution.

#### Scenario: Stronger claim requested

- **GIVEN** all proofs pass
- **WHEN** evidence claims the store's roots were complete or the removals executed correctly
- **THEN** the claim MUST be rejected as outside the proof boundary
