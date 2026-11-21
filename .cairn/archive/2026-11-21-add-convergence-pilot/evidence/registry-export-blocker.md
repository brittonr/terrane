# Requirement-registry export is blocked by duplicate IDs

Command (cairn-convergence binary at Cairn 830b544, committed revision
`a79e3d18a90721954d5e2484d8ed655d497fabfd`):

```console
cairn requirement-registry export --root . --repository-id onixresearch/terrane \
  --revision-kind git --revision a79e3d1... --out proof-rails/verus-bounds/requirement-registry.json
```

Exit 1; no registry file is written. The verdict carries nine
`content_requirement_registry.requirement_id_duplicate` issues:

| ID | Definitions |
|---|---|
| `bounds.in-range` | `.cairn/specs/bounds/spec.md:13`, `.cairn/specs/fetch-add/spec.md:14` |
| `cache-partition.assignment` | `.cairn/specs/cache-partitioning/spec.md:72`, `.cairn/specs/control-flow-graph/spec.md:31` |
| `cache-partition.isolation` | `.cairn/specs/cache-partitioning/spec.md:13`, `.cairn/specs/set-associative-lookup/spec.md:49` |
| `chain.collision-free` | `.cairn/specs/chain/spec.md:58`, `.cairn/specs/constant-folding/spec.md:53` |
| `constfold.finite-height` | `.cairn/specs/constant-folding/spec.md:13,42` |
| `dce.side-effect-never-dead` | `.cairn/specs/dead-code-elimination/spec.md:61,67` |
| `fetch-add.associativity` | `.cairn/specs/fetch-add/spec.md:49,67` |
| `liveness.exit` | `.cairn/specs/control-flow-graph/spec.md:120`, `.cairn/specs/liveness-analysis/spec.md:88` |
| `loop.header-dominance` | `.cairn/specs/loop-detection/spec.md:38,143` |

Eight of the nine are pre-existing: the same command at base `b9bd540` in a
detached worktree exits 1 with eight duplicate issues at the same ids (all
except `bounds.in-range`, which the new bounds spec now also declares while
`.cairn/specs/fetch-add/spec.md` still carries its copy-paste
`r[bounds.in-range]` line under "Return Semantics").

WP8.7 needs an exportable requirement registry. Repairing the unrelated
domains is outside this change's scope, so the export, `targets.ncl`,
`contract.ncl`, `generated.ncl` and their JSON were not produced.
