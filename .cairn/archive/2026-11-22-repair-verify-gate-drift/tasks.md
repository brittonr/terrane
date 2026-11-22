## Baseline

- [x] [serial] Capture the five failing `./scripts/verify.rs` sections, their exact diagnostics, and the per-checker exit codes before any refresh. r[verification-tooling.recorded-evidence-refresh]
  - Log: `evidence/verify-baseline.txt` (the first run only reached the checker shebang; `evidence/verify-baseline-run.txt` records the gate itself).

## Recorded evidence refresh

- [x] [serial] Refresh the `trellis.scheme.reader.structure` source identity in `docs/proof-artifact-manifest.json` and re-bind `docs/proof-artifact-manifest.preserves-envelope.json` to the resulting manifest digest, then run the freshness checker and its self-test. r[verification-tooling.recorded-evidence-refresh]
- [x] [serial] Regenerate `nickel-export-cutover/generated/outputs/proof-evidence-contract.json` through `./scripts/check-nickel-export-cutover.rs --write` and re-run `--check` and `--self-test`. r[verification-tooling.recorded-evidence-refresh]
- [x] [serial] Regenerate `aeneas-pilot/generated/aeneas-proof-receipt.json` and `cross-verifier-manifest.json` through `./scripts/check-aeneas-cross-verifier.rs --write` and re-run `--check` and `--self-test`. r[verification-tooling.recorded-evidence-refresh]
- [x] [parallel] Validate all eight `docs/aspen-verified-logic-index.tsv` rows against a sibling `../aspen` checkout at the documented path without changing the index or the checker. r[verification-tooling.recorded-evidence-refresh]

## Pin boundary

- [x] [serial] Record the consumer release-profile drift, its root cause at revision `df99309`, and the exact `inputs.trellis-consumer-release-src` pin move it would need, leaving the profile, every flake input, and `flake.lock` unchanged. r[verification-tooling.recorded-evidence-refresh]

## Validation and lifecycle

- [x] [serial] Run `./scripts/verify.rs`, `./scripts/verus.sh src/lib.rs`, and `tracey query uncovered`/`untested`; capture the exit codes and the remaining pin-bound failure verbatim. r[verification-tooling.recorded-evidence-refresh]
- [x] [serial] Sync the accepted spec, archive the change with `CAIRN_ARCHIVE_DATE`, and re-run validation plus the touched checkers post-archive. r[verification-tooling.recorded-evidence-refresh]
