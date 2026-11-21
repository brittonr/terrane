# loop-detection dedup delta

## MODIFIED Requirements

### Requirement: Exit Edges
r[loop.exits]

All edges from loop body nodes to non-body nodes MUST be identified as
exit edges.

#### Scenario: Exit edge detected

- GIVEN edge (S → T) where S is in the loop body and T is not
- WHEN exit edges are listed
- THEN this edge is included

#### Scenario: Internal edge excluded

- GIVEN edge (S → T) where both S and T are in the loop body
- WHEN exit edges are listed
- THEN this edge is not included
