# Aspen evidence index drift is pre-existing

At HEAD, `./scripts/verify.rs` reports
`✗ Aspen evidence index drift detected`. The same check was run in a temporary
Git worktree at base `b9bd540` with its own fresh target directory:

```console
git worktree add --detach /tmp/terrane-wp8-base b9bd540
cd /tmp/terrane-wp8-base
nix --option min-free 0 --option builders '' develop .# -c bash -c \
  'CARGO_TARGET_DIR=/tmp/terrane-base-target RUSTC_WRAPPER= ./scripts/check-aspen-evidence-index.rs'
```

Exit 1; transcript `../convergence-evidence/wp8/transcripts/aspen-base.txt`:

```
aspen evidence index drift detected
row coordination-rate-limiter-refill-acquire: source repo ././../aspen is not a git checkout
```

The drift comes from the check resolving `././../aspen` relative to the
worktree and finding no git checkout there. It is reproducible at the base
revision and is unrelated to the convergence-pilot files.
