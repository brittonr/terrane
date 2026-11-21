# set-associative-lookup dedup delta

## MODIFIED Requirements

### Requirement: Hit Correctness
r[set-assoc.hit]

A cache hit MUST mean the tag stored in the matched way equals the
requested tag.

#### Scenario: Hit returns correct way

- GIVEN a set with tag T stored in way W
- WHEN a lookup for tag T is performed
- THEN the result is hit at way W

#### Scenario: Hit identifies requested line

- GIVEN a hit at way W
- WHEN the way's stored tag is inspected
- THEN it equals the requested tag and therefore identifies the requested cache line
