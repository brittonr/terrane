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

## E2 facts to fill (placeholders)

Each token below appears where the value is required. No value may be guessed;
all come from E2's records.

| Placeholder | Source |
|---|---|
| `E2_PHYSICAL_ROOT` | Mantle physical store path of the built root |
| `E2_TARGET_PATH` | built root store path |
| `E2_NODE_ID` | realization unit `node_id` (`nix:` + built root drv name) |
| `E2_TARGET_DERIVATION` | realization unit `target_derivation` |
| `E2_UNIT_INDEX` | index of the unit whose `node_id` matches |
| `E2_RECEIPT_BLAKE3`, `E2_PLAN_BLAKE3`, `E2_IMPORT_RECEIPT_BLAKE3` | preserve receipt fields |
| `E2_STORE_NAR_SHA256`, `E2_SIGNER` | store-info record |
| `E2_MANTLE_BLAKE3`, `E2_BRIDGE_BLAKE3` | checked executable digests |
| `E2_HANDOFF_RECEIPT`, `E2_HANDOFF_ASSOCIATION`, `E2_HANDOFF_JOB`, `E2_HANDOFF_GRAPH`, `E2_HANDOFF_REPORT` | `handoff-built/mantle-observation.json` and report |
| `E2_CACHE_PUBLIC_KEY_NAME` | WP8 key name (secret never recorded) |
| `E2_NAR_MATCH`, `E2_DIFF_CLEAN` | NAR comparison and `diff -r` result |
| `E2_DECISION_BLAKE3`, `E2_VERIFICATION` | Octet decision and verify result |
| `E2_CONVERGENCE_RECEIPT_BLAKE3` | new inspect receipt |
| `E2_NEGATIVES` | `20/20` when all per-binding records behave |

## Risks / Trade-offs

- If E2's built bytes differ from the Nix output, the built root stays the
  evidence subject and the difference is recorded as a determinism finding.
- The case grows by the built output copies; the producer-output copy remains
  the same size class as the preserve case.
