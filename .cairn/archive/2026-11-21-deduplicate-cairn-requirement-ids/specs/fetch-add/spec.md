# fetch-add dedup delta

## MODIFIED Requirements

### Requirement: Return Semantics
r[fetch-add.semantics]

FAA MUST return the value that was in the cell before the addition and
store the sum in the cell.

#### Scenario: Wrapping add

- GIVEN a cell containing value `c` and an addend `a`
- WHEN `fetch_add_wrapping(c, a)` is called
- THEN the returned value is `c` and the new cell content is `(c + a) mod 2^64`

#### Scenario: Saturating add

- GIVEN a cell containing value `c` and an addend `a`
- WHEN `fetch_add_saturating(c, a)` is called
- THEN the returned value is `c` and the new cell content is `min(c + a, u64::MAX)`


### Requirement: Commutativity
r[fetch-add.commutativity]

Two sequential FAAs MUST produce the same final cell value regardless of
the order the addends are applied.

#### Scenario: Wrapping commutativity

- GIVEN a cell containing `c`, addends `a` and `b`
- WHEN `fetch_add_wrapping(c, a)` then `fetch_add_wrapping(result, b)` is performed
- THEN the final cell value equals that of `fetch_add_wrapping(c, b)` then `fetch_add_wrapping(result, a)`

#### Scenario: Saturating commutativity

- GIVEN a cell containing `c`, addends `a` and `b`
- WHEN `fetch_add_saturating(c, a)` then `fetch_add_saturating(result, b)` is performed
- THEN the final cell value equals that of `fetch_add_saturating(c, b)` then `fetch_add_saturating(result, a)`
