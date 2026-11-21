# WP8.8 rehearsal (not final evidence)

Run over the Nix-built producer output, not over a Mantle-realized output:

```console
tools/octet-judge/run.sh /tmp/octet-judge/bin/cargo-octet \
  proof-rails/verus-bounds/contract.json \
  /nix/store/9m8and9qpxdhm9mdv35yhncxj83gss21-trellis-bounds-verus-job \
  /home/brittonr/wt/convergence-evidence/wp8/rehearsal
```

`requirements check` exit 0 with `aggregate_result = "passed"` and no
diagnostics; `requirements verify` recomputed `status = "passed"`,
`aggregate_result = "passed"`, zero diagnostics. Decision BLAKE3
`d1722f409a8e703b645e2a3bac4a3f62c25d986a5d53f97f4478de943bed2665`.

Negatives (`check-judge-negatives.sh`) all refused: wrong contract byte
(allowed evidence id changed), missing proof card, provider mismatch
(`aeneas-lean` required while the envelope names `verus-v1-compatibility`),
and empirical evidence offered for the formal clause.

The rehearsal exposed and fixed three producer/contract defects: card and
evidence digests now use plain BLAKE3 hex (Octet compares unprefixed hex), the
job output carries `.cairn/specs/bounds/spec.md` for the `--source-root`
specification check, and the contract id matches the producer evidence id
(`trellis.bounds.verus`).

Transcripts: `/home/brittonr/wt/convergence-evidence/wp8/rehearsal/`
(`decision.json`, `verification.json`, `negatives.txt`). These are rehearsal
records only; the final judgment must run over the Mantle-realized output.
