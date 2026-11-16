# Design: Add reclaim plan kernel

## Decisions

### Decision: Checker first, planner second

**Choice:** Validity is a one-pass, local property: roots retained, retention
closed under references, candidates exact, and protected nodes never
referencing reclaimable ones. The safety theorems follow from validity alone,
so any planner whose output passes `check_plan` is safe.

**Rationale:** A consumer can validate its own planner's output without
trusting it. The checker cannot rule out over-retention, so the no-leak
property needs the verified planner's soundness invariant.

### Decision: Fixpoint iteration with a round cap

**Choice:** The planner propagates retention across all references until a
round makes no change, capped at `n + 1` rounds. Reaching the cap returns
`ROUND_LIMIT` with no plan.

**Rationale:** A round without change gives closure directly, avoiding a
breadth-first queue's completeness invariant. A counting argument proves the
cap is never reached: every changing round retains at least one more node, and
at most `n` nodes exist. `ROUND_LIMIT` therefore stays only as unreachable
fail-closed code.

### Decision: Total functions over indices

**Choice:** Every exec function is total. Out-of-range roots or references and
oversized inputs reject with a class instead of being preconditions.

**Rationale:** A trusted precondition is a hole a consumer can fall through.
