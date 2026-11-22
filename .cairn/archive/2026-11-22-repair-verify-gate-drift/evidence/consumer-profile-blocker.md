# Consumer release profile: blocked on a flake pin move

Command (inside the dev shell):

```
$ ./scripts/check-verified-logic-consumer-profile.rs
error: profile.artifact.drift: Cargo.toml expected 5685:2fa954e69796c98f09233ef445752d3ae6a953feb061ef75ff35724bff5bb87b got 5685:cdcde7fa8e244f6cd4da336dc25d08c0937cf7cfae218f00b1ec66ad386d3808
error: profile.artifact.drift: src/lib.rs expected 52890:47386127f1e4f3a5db3c161b334e0a6eb3176b79a2ae6adbac1b4f779e17c467 got 52890:4b297b4ad379c7bd58fbb5a82826660e3292cfdfcef7a20270303d702f0d1676
error: profile.artifact.drift: src/ucan_backend_transcript.rs expected 5925:e3bfdd1fed32d657dc836d2a9b8478225233eff2be916d0e9e4bee3a7e5efe1b got 6615:dfafe2dc9e5e4f168ea82d8541a6f01c391b13c35dbc96131066e1ad6603fa17
error: profile.verification.proof_manifest.drift
EXIT=1
```

## Root cause

`consumer-profile/profile.ncl` binds revision `df99309c31e95b8068d90153e362dad6234d0cca`
and its artifact rows match that tree exactly:

```
$ git cat-file blob df99309:Cargo.toml | b3sum --no-names
2fa954e69796c98f09233ef445752d3ae6a953feb061ef75ff35724bff5bb87b   (5685 bytes)
$ git cat-file blob df99309:src/lib.rs | b3sum --no-names
47386127f1e4f3a5db3c161b334e0a6eb3176b79a2ae6adbac1b4f779e17c467   (52890 bytes)
$ git cat-file blob df99309:src/ucan_backend_transcript.rs | b3sum --no-names
e3bfdd1fed32d657dc836d2a9b8478225233eff2be916d0e9e4bee3a7e5efe1b   (5925 bytes)
```

Commit `96c5282` changed all three files (and `docs/proof-evidence-bundle.json`),
so the worktree no longer equals the recorded release revision.

`scripts/check-verified-logic-consumer-profile.rs` measures `source_root`, which
defaults to the repository root (the worktree). The Nix check
`checks.<system>.consumer-release-profile` in `flake.nix` runs the same checker
with `--source-root ${trellis-consumer-release-src}` and
`--source-revision ${trellis-consumer-release-src.rev}`, and
`validate_observed_source_revision` requires the observed revision to equal the
profile revision. Both gates can therefore only pass together when the profile
revision equals the worktree revision.

## Pin it would need

`flake.nix:40` input `trellis-consumer-release-src`
(`git+https://github.com/OnixResearch/terrane.git?rev=df99309c31e95b8068d90153e362dad6234d0cca`,
locked in `flake.lock`) would have to move to the new published release revision,
together with `profile.ncl` (`release_id`, `package.revision`,
`dependency_coordinate`), every artifact row, `verification.proof_manifest_bytes`
/ `proof_manifest_blake3` (the manifest digest is now
`a42a24381b33737810cde95437fc7a3419e657d9b91d2071dc78f8ebf155dd33` / 21530 bytes),
`consumer-profile/generated/profile.json`, and
`docs/verified-logic-consumer-profile.md`.

That is a pin move plus a `flake.lock` update, both out of scope for this change,
so the profile is left untouched. Rewriting only the artifact rows would move the
failure onto the Nix gate and would misdescribe revision `df99309`.
