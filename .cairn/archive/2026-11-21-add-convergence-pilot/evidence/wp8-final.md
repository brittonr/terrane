# Final judgment and convergence receipt

Producer source revision `e8a961af3dbeb6f7a0f7dfbf82100bfe2abcb482`. Job output
`/nix/store/5hqkrl54nhfa44nmmaf80phvyi5kvjz1-trellis-bounds-verus-job`, root
derivation `/nix/store/cvgjzlyd2nn1ihbq4z4s5ns6slirx7is-trellis-bounds-verus-job.drv`.

## Octet judgment

`tools/octet-judge/run.sh` over the Mantle-preserved root
(`/datapool/mantle-store/wp8-20261121/store-preserve-2/5hqkrl54…-trellis-bounds-verus-job`):
`aggregate_result = passed`, clause `bounds.clamp = passed`, decision BLAKE3
`aaa7f5a47a9b2fe68681d3f9eb6e8381bc1f33d27ead4e26622a15fad4992819`.
`requirements verify` recomputed `passed/passed` with no diagnostics.

## Cairn convergence

`cairn convergence inspect --case proof-rails/verus-bounds/convergence/case.json
--root <checkout at e8a961a>`: `status: evaluated`, `receipt_blake3
5de736a0597c8b7d87517885c7ce2c78f8d6cfd1445a84be244035d31ddedf89`,
`targets_current: true`, `complete: false` (all other rows are untargeted).
`bounds.clamp` is `current`, bound to root
`/nix/store/5hqkrl54nhfa44nmmaf80phvyi5kvjz1-trellis-bounds-verus-job`, unit
`nix:cvgjzlyd2nn1ihbq4z4s5ns6slirx7is-trellis-bounds-verus-job.drv`, route
`cache-only-preserve-v1`, signer `wp8-20261121-1`.

Twenty negative records in `convergence/negatives/` each reproduce their
expected state and issue codes (`20/20`).

## Realization non-claim (verbatim)

"The pilot output was built by Nix and preserved through Mantle's cache-only
route; Mantle did not build it. The real build route (Mantle building the root
from a Nix-cache-hydrated dependency closure) is unsupported at T2."

A cache hit preserves an earlier acceptance and is not fresh proof work.
Octet's decision is the only acceptance; Cairn binds saved records and
authenticates none of them.
