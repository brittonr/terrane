# constant-folding dedup delta

## MODIFIED Requirements

### Requirement: Lattice Properties
r[constfold.lattice]

The constant propagation lattice MUST satisfy bounded lattice axioms.

#### Scenario: Commutativity

- GIVEN lattice values A and B
- WHEN `meet(A, B)` and `meet(B, A)` are computed
- THEN the results are equal

#### Scenario: Associativity

- GIVEN lattice values A, B, C
- WHEN `meet(meet(A, B), C)` and `meet(A, meet(B, C))` are computed
- THEN the results are equal

#### Scenario: Idempotence

- GIVEN lattice value A
- WHEN `meet(A, A)` is computed
- THEN the result is A

#### Scenario: Top identity

- GIVEN any lattice value A
- WHEN `meet(Top, A)` is computed
- THEN the result is A


### Requirement: Transfer Function Monotonicity
r[constfold.monotone]

The transfer function MUST be monotone with respect to the lattice ordering.

#### Scenario: Monotone

- GIVEN input environments env1 ≤ env2 (pointwise lattice ordering)
- WHEN `transfer(instr, env1)` and `transfer(instr, env2)` are compared
- THEN `transfer(instr, env1) ≤ transfer(instr, env2)`
