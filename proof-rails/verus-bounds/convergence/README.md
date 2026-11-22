# Bounds convergence case (Mantle-built root)

Registry revision `e8a961af3dbeb6f7a0f7dfbf82100bfe2abcb482`. The case binds
the root Mantle built with the `cache-inputs-build-roots-v1` route:
`/nix/store/5hqkrl54nhfa44nmmaf80phvyi5kvjz1-trellis-bounds-verus-job`, physical
store `/datapool/mantle-store/wp8-20261121/realize-evidence-store/…`, NAR
`sha256-zUn6DMJIENyxoINDZMGptW0FLphtmE0ydx9mfM1ft1o=` equal to the Nix-built
NAR with an empty `diff -r`.

Inspect with the pinned Cairn binary:

```sh
cairn convergence inspect --case proof-rails/verus-bounds/convergence/case.json \
  --root <checkout at e8a961af…>
```

The receipt reports `bounds.clamp` `current` and `targets_current: true`.

## Trust boundary

Mantle built the root from inputs hydrated from the WP8-signed cache with the
checked `mantle` executable built from Mantle `5263dabfc…`
(`a3cf39241e2df5c8b7bbc3cfdd37e1dd5fa28255dbbf1278c62b0d84f2044762`); the
bridge is from a clean export of the same commit
(`604da7524cf3290ba18702f05beffa418ac74278c2b09644c955753919df8d3d`).

Mantle does not rebuild the hydrated inputs; their trust is the WP8 signing key
and the Nix cache. The bridge does not evaluate proofs. Octet's decision is the
only acceptance; Cairn binds saved records and authenticates none of them.

`realization-receipt.json`: `cache-inputs-build-roots-v1`, `complete`/`realized`,
receipt `48e0bea9…`, 1 built (the root), 264 substituted, 12 already-present,
7,247 not-required. `store-info.json` is signed by `crunch-britton-desktop-1`.
