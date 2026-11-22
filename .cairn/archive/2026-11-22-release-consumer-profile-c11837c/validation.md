# Validation Evidence

## Bound identities

- Base revision: `c11837c5c6cdf2714fd9015da07196d53eea28eb` (published `main`, branch `cairn/consumer-release-c11837c`)
- `flake.nix` input `trellis-consumer-release-src`: `git+https://github.com/OnixResearch/terrane.git?rev=c11837c5c6cdf2714fd9015da07196d53eea28eb`
- Locked `flake.lock` revision: `c11837c5c6cdf2714fd9015da07196d53eea28eb`, `narHash sha256-BbUK96Q0pcScjMFmm4gbkvY+Rd5A/Ns3tziNTtkNt1w=`
- Profile `release_id`: `verified-logic-0.2.0-ucan-profile-df99309` → `verified-logic-0.2.0-ucan-profile-c11837c`
- Profile dependency coordinate: `rev=df99309…` → `rev=c11837c5c6cdf2714fd9015da07196d53eea28eb`
- Profile: 39 UCAN-facing modules and 42 selected artifacts (unchanged)
- Proof manifest: `docs/proof-artifact-manifest.json`, 21530 bytes, BLAKE3 `a42a24381b33737810cde95437fc7a3419e657d9b91d2071dc78f8ebf155dd33`

## Measured artifact drift at `c11837c5`

Measured with `git cat-file -s` and `git cat-file blob <rev>:<path> | b3sum`:

| Path | Old bytes / BLAKE3 | New bytes / BLAKE3 |
| --- | --- | --- |
| `Cargo.toml` | 5685 / `2fa954e6…f5dfbb87b` | 5685 / `cdcde7fa8e244f6cd4da336dc25d08c0937cf7cfae218f00b1ec66ad386d3808` |
| `src/lib.rs` | 52890 / `47386127…4f779e17c467` | 52890 / `4b297b4ad379c7bd58fbb5a82826660e3292cfdfcef7a20270303d702f0d1676` |
| `src/ucan_backend_transcript.rs` | 5925 / `e3bfdd1f…e5efe1b` | 6615 / `dfafe2dc9e5e4f168ea82d8541a6f01c391b13c35dbc96131066e1ad6603fa17` |
| `docs/proof-artifact-manifest.json` | 21530 / `93d4105b…bfd070002` | 21530 / `a42a24381b33737810cde95437fc7a3419e657d9b91d2071dc78f8ebf155dd33` |

The remaining 39 artifact rows are unchanged between `df99309` and `c11837c5`.

## Focused profile validation

- `nix --option min-free 0 --option builders '' flake lock --update-input trellis-consumer-release-src` → exit 0, `git diff flake.lock` touches only the `trellis-consumer-release-src` node.
- `nickel export --format json consumer-profile/profile.ncl` regenerated `consumer-profile/generated/profile.json`.
- `./scripts/check-verified-logic-consumer-profile.rs` → exit 0, `passed: 39 modules, 42 artifacts, 10 fixtures`.
- `nix --option min-free 0 --option builders '' build .#checks.x86_64-linux.consumer-release-profile --no-link -L` → exit 0, the check's own run reports `passed: 39 modules, 42 artifacts, 10 fixtures` and `self-test passed`.

## Repository validation

- `./scripts/verus.sh src/lib.rs --rlimit 20` → exit 0, `verification results:: 11139 verified, 0 errors`.
- `tracey query uncovered` → `0 uncovered out of 2270 rules`.
- `tracey query untested` → `0 untested out of 2270 rules`.
- `nix --option min-free 0 --option builders '' develop -c ./scripts/verify.rs` → exit 0 with `ALL CHECKS PASSED` and every section green (run inside the dev shell with `RUSTC_WRAPPER=` and a fresh `CARGO_TARGET_DIR`; a rustup-shim `rustc` inherited outside the dev shell makes the cargo-script checkers reuse artifacts from an incompatible compiler).

## Non-claims

These checks establish exact release/profile, pin, and selected-source alignment only. They do not prove UCAN token correctness, downstream authorization policy, verifier soundness beyond Terrane's stated boundary, external consumer readiness, or whole-system security.
