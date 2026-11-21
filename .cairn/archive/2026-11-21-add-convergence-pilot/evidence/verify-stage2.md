# Stage 2 verify run with a fresh target

```console
NIX_CONFIG=$'builders = \nmin-free = 0' nix --option min-free 0 --option builders '' develop .# -c bash -c \
  'CARGO_TARGET_DIR=/tmp/terrane-verify-target RUSTC_WRAPPER= ./scripts/verify.rs'
```

Exit 1. Transcript: `/home/brittonr/wt/convergence-evidence/wp8/transcripts/verify-fresh-target-final.txt`.

No `E0514` occurs with the fresh target. The remaining failures are all
reproduced at base `b9bd540` in a temporary worktree with its own fresh target
(`transcripts/base-unrelated-checks.txt`):

| Check | Base exit | Same diagnostic |
|---|---|---|
| Aspen evidence index | 1 | `row coordination-rate-limiter-refill-acquire: source repo ././../aspen is not a git checkout` |
| Nickel export cutover | 1 | `legacy destination drift for proof-evidence-contract` |
| Aeneas cross-verifier | 1 | `proof receipt does not match current profile and artifacts` |
| Verified primitive manifest | 1 | `src/scheme_logic/syntax.rs expected 9914 bytes, measured 9945 bytes` |
| Verified-logic consumer profile | 1 | identical `profile.artifact.drift` rows for `Cargo.toml`, `src/lib.rs`, `src/ucan_backend_transcript.rs` |

Checks on the touched surface pass in the same run: Aeneas frontier
(`596 modules, 2270 requirements, 5 targeted`), proof evidence bundle and IR
chain, 5223 library tests plus the checker suites, Verus proofs, and Tracey
(`0 uncovered`, `0 untested`, 2270 rules).
