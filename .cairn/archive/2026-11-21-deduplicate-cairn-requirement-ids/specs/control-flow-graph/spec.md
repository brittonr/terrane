# control-flow-graph dedup delta

## MODIFIED Requirements

### Requirement: Instruction Partition
r[control-flow-graph.partition]

Every instruction MUST belong to exactly one basic block, and blocks
MUST partition the instruction sequence.

#### Scenario: Complete coverage

- GIVEN an instruction sequence of length N
- WHEN the blocks are checked
- THEN every index in [0, N) belongs to some block

#### Scenario: No overlap

- GIVEN two distinct blocks B1 and B2
- WHEN their instruction ranges are compared
- THEN the ranges do not overlap

#### Scenario: No gaps

- GIVEN consecutive blocks B_i and B_{i+1} sorted by start
- WHEN their boundaries are checked
- THEN B_i.end == B_{i+1}.start


### Requirement: Exit Block
r[control-flow-graph.exit-block]

Exit blocks MUST end with return instructions and have no successors.

#### Scenario: Return terminates

- GIVEN a block ending with a return instruction
- WHEN successors are listed
- THEN the list is empty

#### Scenario: All returns are exits

- GIVEN every return instruction in the program
- WHEN its containing block is checked
- THEN the block is an exit block
