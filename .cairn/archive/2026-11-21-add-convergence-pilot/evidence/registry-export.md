# WP8.7 registry and targets

Exported at committed revision `59fdf9c58cac89b79152a0e2b0fcd8300a654cc3` with the
`cairn-convergence` binary (Cairn 830b544):

```console
cairn requirement-registry export --root . --repository-id onixresearch/terrane \
  --revision-kind git --revision 59fdf9c... --out proof-rails/verus-bounds/requirement-registry.json
cairn requirement-registry verify --root . --registry proof-rails/verus-bounds/requirement-registry.json \
  --repository-id onixresearch/terrane
```

Export exit 0 (`valid: true`, zero issues); verify exit 0 (`verdict: pass`).
1603 rows.

`bounds.clamp` row:

```json
{
  "requirement_id": "bounds.clamp",
  "specification_path": ".cairn/specs/bounds/spec.md",
  "specification_blake3": "b3800b8fee7352fc83cd98a3386325a8fe2d45c1d2b4385434461027c199291c",
  "source_span": {
    "start_byte": 974,
    "end_byte": 1126,
    "start_line": 37,
    "end_line": 40
  },
  "requirement_body_blake3": "29b0e86db1211657c578b2e447133841058b588d448365dba8555b4f9e3300eb",
  "row_blake3": "b7db99df29a5865823e6c261453a6a1d4877fb89d54784d6c84d0ecd0d789c23"
}
```

Outputs: `targets.ncl`/`targets.json` (row derived with
`requirement_from_registry` and `specification_from_registry`),
`generated.ncl`/`generated-input.json`/`generated.json`, and
`contract.ncl`/`contract.json` (`octet-requirement-contract/v1`, disposition
`formal_proof`, required provider `verus-v1-compatibility`, required evidence
domain:

