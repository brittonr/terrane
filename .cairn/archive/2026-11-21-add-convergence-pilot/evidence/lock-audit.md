# flake.lock audit

All 12 pre-existing root inputs resolve to identical locked trees (cairn,
crane, flake-utils, nickel-1-17-0, nickel-export, nixpkgs, octet,
octet-aeneas, onixpkgs, rust-overlay, trellis-consumer-release-src,
verus-upgrade-trial-src). The changed `llm-agents`, `root` and
`rust-overlay_16` nodes come from Nix renumbering; the only real additions are
the `octet-judge` input and its transitive subtree.

An independent check over the same diff produced:
`existing root inputs with changed resolved content: []`, `removed: []`.
