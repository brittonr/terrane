# Final verify run

`./scripts/verify.rs` in the dev shell with
`CARGO_TARGET_DIR=/tmp/terrane-verify-target-2` and no remote builders.
Transcript: `/home/brittonr/wt/convergence-evidence/wp8/transcripts/verify-final.txt`.

The same pre-existing base failures recorded in `verify-stage2.md` persist and
are unrelated to this change: Aspen evidence index drift (base exit 1),
standalone Nickel export cutover legacy destination drift, Aeneas
cross-verifier receipt mismatch, verified primitive manifest stale rows for
`src/scheme_logic/syntax.rs`, and consumer-profile artifact drift. All were
reproduced at base `b9bd540` with its own fresh target
(`transcripts/base-unrelated-checks.txt`).

Tracey reports `0 uncovered` and `0 untested` of 2270 rules.
