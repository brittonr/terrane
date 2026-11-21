# Design: Deduplicate Cairn requirement IDs

## Context

Nine IDs are declared by two requirement blocks. The registry binds a row by
ID and rejects duplicates. Each pair has one block that matches the ID's domain
and heading and one copy-paste block that does not. Delta merge matches
MODIFIED operations by stable ID, so an ID rename is expressed as a REMOVED
old-ID block plus an ADDED new-ID block; the rewritten block moves to the end
of its accepted spec.

## Rename table

| Old ID | Spec:line | Heading | New ID | Source |
|---|---|---|---|---|
| `bounds.in-range` | `fetch-add/spec.md:14` | Return Semantics | `fetch-add.semantics` | 2a |
| `fetch-add.associativity` | `fetch-add/spec.md:49` | Commutativity | `fetch-add.commutativity` | 2a |
| `cache-partition.assignment` | `control-flow-graph/spec.md:31` | Instruction Partition | `control-flow-graph.partition` | 2a |
| `liveness.exit` | `control-flow-graph/spec.md:120` | Exit Block | `control-flow-graph.exit-block` | 2b |
| `cache-partition.isolation` | `set-associative-lookup/spec.md:49` | Hit Correctness | `set-assoc.hit` | 2a |
| `constfold.finite-height` | `constant-folding/spec.md:13` | Lattice Properties | `constfold.lattice` | 2a |
| `chain.collision-free` | `constant-folding/spec.md:53` | Transfer Function Monotonicity | `constfold.monotone` | 2a |
| `dce.side-effect-never-dead` | `dead-code-elimination/spec.md:67` | Live Definitions Never Dead | `dce.live-def-never-dead` | 2a |
| `loop.header-dominance` | `loop-detection/spec.md:143` | Exit Edges | `loop.exits` | 2a |

`control-flow-graph.exit-block` uses rule 2b: the canonical
`control-flow-graph.exit` exists in `docs/requirements.md`, but that ID is
already bound in the same accepted spec to the unrelated "Reachability" block
at `control-flow-graph/spec.md:154`. Reusing it in this delta would make the
operation IDs conflict. The code marker `r[impl control-flow-graph.exit]` in
`src/control_flow_graph.rs` continues to resolve to the canonical docs ID and
is not changed.

## Decisions

### Decision: Delta specs, not hand-edited accepted specs

**Choice:** express every rename as a REMOVED block plus an ADDED block in
`.cairn/changes/deduplicate-cairn-requirement-ids/specs/<domain>/spec.md`.

**Rationale:** Terrane's Cairn lifecycle owns accepted-spec rewrites through
sync and archive. Requirement text, scenarios and heading lines are copied
byte-for-byte; only the `r[...]` line changes.

## Risks / Trade-offs

- Each moved block appends to the end of its accepted spec, so requirement
  order within those six specs changes once.
- The pre-existing "Reachability" block labelled `control-flow-graph.exit` at
  `control-flow-graph/spec.md:154` is left unchanged: it is not one of the nine
  duplicates, and renaming it would conflict with reusing the canonical ID in
  the same delta.

## Note 2026-11-21: actual repair path

This section is an appended correction; the decision text above is unchanged.

`cairn sync` refused the planned REMOVED+ADDED deltas before any accepted
bytes changed, with
`delta_merge.duplicate_accepted_id: accepted requirement ID
'fetch-add.associativity' occurs in conflicting blocks`. Delta merge
preflights the accepted document, so it cannot target an ID that two blocks
already share. The nine requirement ID lines were therefore edited directly in
`.cairn/specs/<domain>/spec.md`; only the `r[...]` line moved. The change's
MODIFIED deltas then synced as `already_applied` with `blocked: false` and
`changed: false`, and `cairn archive` recorded one action. No requirement
heading, body or scenario changed.

Product observation, not a claim: at this revision Cairn's delta merge cannot
repair an accepted requirement ID that already occurs in two blocks, because
the preflight refuses the accepted document before operations are applied. A
one-time direct correction, recorded in change evidence, was the available
path.
