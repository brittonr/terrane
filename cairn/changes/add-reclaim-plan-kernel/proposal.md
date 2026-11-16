# Change: Add reclaim plan kernel

## Why

A garbage collector's plan is a list of removals that cannot be undone. Its
safety rests on two facts: nothing a root reaches is removed, and nothing that
survives references something removed. Terrane's `reachability` module proves
that marked nodes are reachable, but not the converse, which is the direction a
collector's safety depends on.

## What Changes

- Add a `reclaim_plan` module over an indexed reference graph with roots and a
  reclaimable flag per node.
- Add `check_plan`, proved equal to a validity specification for any plan, and
  prove from validity alone that no candidate is live, that no surviving node
  references a removed one, and that protected nodes are never candidates.
- Add `plan`, a fixpoint planner proved to return only valid plans whose
  retained nodes are all reachable, so every unreachable reclaimable node is a
  candidate, and that admissible input always plans.
- Add positive, negative, and exhaustive tests over every graph with up to
  three nodes against an independent oracle.

## Non-Goals

- Whether the caller's roots and references are complete or current.
- The mapping from store paths to indices, plan identities, byte totals, and
  the execution of removals.

## Impact

- **Affected spec:** `reclaim-plan`
- **Affected code:** one verified module, `src/lib.rs`, `docs/requirements.md`,
  and focused tests
- **Consumers:** Mantle can check its garbage-collection plans against this
  model through a fail-closed projection
