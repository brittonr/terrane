# E2 realization facts

Route `cache-inputs-build-roots-v1`, fresh physical store, WP8-signed cache
with both WP8 root outputs removed. Receipt
`/home/brittonr/wt/convergence-evidence/wp8/build-roots/realization-receipt.json`:
`complete`/`realized`, `receipt_blake3 48e0bea9…`, `plan_blake3 2183efc5…`,
`import_receipt_blake3 f719f54d…`, `build_report_blake3 96ee9801…`, unit 7523
`nix:cvgjzlyd…` `execution_class: built`, disposition counts 1 built / 264
substituted / 12 already-present / 7,247 not-required. No non-root unit was
built.

Built root `/nix/store/5hqkrl54…-trellis-bounds-verus-job` in physical store
`/datapool/mantle-store/wp8-20261121/realize-evidence-store/`; NAR
`sha256-zUn6DMJIENyxoINDZMGptW0FLphtmE0ydx9mfM1ft1o=` equals the Nix NAR and
`diff -r` is empty. `store-info.json` verifies with signer
`crunch-britton-desktop-1`.

Bridge handoff `/home/brittonr/wt/convergence-evidence/wp8/handoff-built/`:
receipt `b3:e1186ca7…`, association `b3:9fcafc24…`, job `b3:387008d0…`, graph
`b3:8bc04fed…`, report `b3:5c2d65ad…`, `proof_acceptance:
not-evaluated-by-mantle`; replay with `PATH=/var/empty` identical.

Final judgment over the built root: `aggregate_result = passed`, decision
`aaa7f5a4…`. Convergence receipt
`bc6bd2c129eff3297b96844627ec6826bd46da066a7e36700ec35ec41e8b30e6`, row
`bounds.clamp` `current`, `targets_current: true`, route
`cache-inputs-build-roots-v1`. 20/20 negatives behaved as specified.

The registry, targets, contract and generated files are unchanged: the
producer source revision is still `e8a961af…` and the reviewed bounds
specification is unchanged.
