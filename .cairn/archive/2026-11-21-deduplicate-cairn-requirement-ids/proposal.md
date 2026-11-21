# Proposal: Deduplicate Cairn requirement IDs

## Why

`.cairn/specs/**/spec.md` declares nine requirement IDs twice. Each pair is a
copy-paste defect: two distinct requirements with different headings and
bodies share one ID. Cairn's content-bound registry requires globally unique
accepted IDs, so `cairn requirement-registry export` exits 1 with
`content_requirement_registry.requirement_id_duplicate` and no registry can be
exported for the convergence pilot.

## What Changes

- Keep each shared ID on the requirement or domain it names.
- Give the other copy the canonical `docs/requirements.md` ID for the same
  requirement, or a unique `<domain>.<heading>` ID when no canonical ID can be
  used. r[dedupe-cairn-requirement-ids.unique]
- Apply the renames only through delta specs so `cairn sync` owns the accepted
  spec rewrite. r[dedupe-cairn-requirement-ids.unique]
- Record every rename with its old ID, location, heading, new ID and source.
  r[dedupe-cairn-requirement-ids.unique]

## Impact

- **Files**: delta specs for `fetch-add`, `control-flow-graph`,
  `set-associative-lookup`, `constant-folding`, `dead-code-elimination` and
  `loop-detection`.
- **Testing**: registry export and verify, Tracey, Cairn validate and gates.
- **Boundary**: requirement bodies, headings and scenarios are unchanged; only
  the ID line moves. Code markers keep the canonical `docs/requirements.md`
  IDs they already use.
