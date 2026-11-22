# Design: Rebinding the case to the Mantle-built root

## Context

The archived case binds `e8a961af…` producer output
`/nix/store/5hqkrl54…-trellis-bounds-verus-job` and the preserve receipt
`86a98c78…`. E2 builds the same frozen root with Mantle's
`cache-inputs-build-roots-v1` route, hydrating inputs from the WP8-signed cache
and building the root with the checked `mantle` executable. The case records
must bind that build.

## Decisions

### Decision: Keep registry, targets, contract and generated files

**Choice:** do not change `requirement-registry.json`, `targets.json`,
`targets.ncl`, `generated*.json/ncl`, `contract.json/ncl` unless E2 changes the
producer source revision or the reviewed specification.

**Rationale:** the producer source revision stays `e8a961af…`, the accepted
bounds specification is unchanged, and the row/target/contract bindings are
independent of which realization route produced the output. A change here would
mean a different producer revision, which this change does not authorize.

### Decision: Bind E2's records exactly

**Choice:** replace `realization-receipt.json`, `store-info.json`, `handoff/`,
the producer-output copies, `decision.json`, `verification.json` and the case's
inspected receipt with the E2 bytes. Copy producer-output receipt/profile
aliases (`external-receipt.json`, `external-profile.json`) from the built root
without changing their bytes.

**Rationale:** Cairn binds exact bytes and identities; the route, unit,
derivation and NAR in the E2 records are the evidence subject. Old records are
not rewritten in place; the superseded preserve case remains in git history.

### Decision: Drop both old non-claims

**Choice:** remove "Mantle did not build it / cache-only preserve" and
"unchecked executables" from the docs, case README (new) and evidence, and
state instead: Mantle built the root from inputs hydrated from the WP8-signed
cache with the checked executable built from Mantle `f7c7fd09…`; the trust of
the hydrated inputs is the WP8 signing key and the Nix cache; Mantle does not
rebuild hydrated inputs.

**Rationale:** the route makes the old non-claims false. The new statement is
the actual trust boundary, not a broader correctness claim.

## E2 facts

| Fact | Value |
|---|---|
| physical root | `/datapool/mantle-store/wp8-20261121/realize-evidence-store/5hqkrl54nhfa44nmmaf80phvyi5kvjz1-trellis-bounds-verus-job` |
| built target path | `/nix/store/5hqkrl54nhfa44nmmaf80phvyi5kvjz1-trellis-bounds-verus-job` |
| node id | `nix:cvgjzlyd2nn1ihbq4z4s5ns6slirx7is-trellis-bounds-verus-job.drv` |
| target derivation | `/nix/store/clnjhb7wy6fgrakdrjdz23ki8ahzap8c-trellis-bounds-verus-job.drv` |
| unit index | 7523 |
| receipt / plan / import / build-report BLAKE3 | `48e0bea9…`, `2183efc5…`, `f719f54d…`, `96ee9801…` |
| store-info NAR / signer | `cd49fa0c…` (`sha256-zUn6DMJIENyxoINDZMGptW0FLphtmE0ydx9mfM1ft1o=`) / `crunch-britton-desktop-1` |
| checked executables | mantle `a3cf3924…` (route code `5263dabfc…`), bridge `604da752…` |
| handoff receipt / association / job / graph / report | `b3:e1186ca7…`, `b3:9fcafc24…`, `b3:387008d0…`, `b3:8bc04fed…`, `b3:5c2d65ad…` |
| route dispositions | 1 built, 264 substituted, 12 already-present, 7,247 not-required |
| cache public key name | WP8 signing key (secret never recorded) |
| NAR match / diff | equal / empty |
| decision / verification | `aaa7f5a4…` / passed |
| convergence receipt | `bc6bd2c129eff3297b96844627ec6826bd46da066a7e36700ec35ec41e8b30e6` |
| negatives | 20/20 behaved as specified |

## Risks / Trade-offs

- If E2's built bytes differ from the Nix output, the built root stays the
  evidence subject and the difference is recorded as a determinism finding.
- The case grows by the built output copies; the producer-output copy remains
  the same size class as the preserve case.
