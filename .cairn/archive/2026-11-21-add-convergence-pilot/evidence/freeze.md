# Producer build and freeze

Built at committed revision `f1c7aee2940b6f4367cd5a1ffd44cc7e38fbde0d`:

```console
nix --option min-free 0 --option builders '' build \
  'git+file:///home/brittonr/wt/terrane-wp8?rev=f1c7aee2940b6f4367cd5a1ffd44cc7e38fbde0d#trellis-bounds-verus-job' -L
```

Exit 0. Output `/nix/store/ppw45bchflda3p08fxr48ws0fankm3n9-trellis-bounds-verus-job`
contains `job.json`, `profile.json`, `provider-profile.json`,
`verus-receipt.json`, `producer-report.json`, `formal-proof-envelope.json`,
`proof-cards.json`, `evidence.json`, `non-claims.json`, `source/`,
`source-revision`, `flake.lock`, and `artifacts/` (stdout, stderr, command,
streams, build context). The producer's own verifier reported
`status: passed` with `checked_documents: 5` and rail
`trellis.bounds.verus#rail-0`.

Freeze at the same revision into
`/home/brittonr/wt/convergence-evidence/wp8/producer-export` printed:

```json
{"root_derivation": "/nix/store/6hjqjawli7gfk1ba3nncdndzwhcyy25y-trellis-bounds-verus-job.drv",
 "source_revision": "f1c7aee2940b6f4367cd5a1ffd44cc7e38fbde0d",
 "graph_format": "nix-derivation-show-v4", "derivations": 7524,
 "source_objects": 629, "source_bytes": 65704832, "proof_executed": false}
```

`README.md` in the same directory lists the job output path and file sizes.
