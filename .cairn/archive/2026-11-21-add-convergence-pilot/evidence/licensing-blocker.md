# WP8.3 licensing blocker at R_C

R_C = Cairn `ffb14c6895fcdccf068fa7d7365f51fdcbc8f74d`.

Cairn's `docs/licensing.md` at R_C assigns:

- `crates/cairn-core/**`, `crates/cairn-gate-contract/**`,
  `crates/cairn-radicle-cob/**`, `templates/**` and reusable lifecycle
  examples: `MIT OR Apache-2.0`;
- policy inputs and generated JSON **named in `cairn-policy/LICENSING.md`**:
  `AGPL-3.0-or-later OR MIT OR Apache-2.0`;
- repository documentation and lifecycle material not carrying another
  notice: `AGPL-3.0-or-later`.

`cairn-policy/LICENSING.md` at R_C names only the five historical files
(`default.ncl`, `contracts.ncl`, `lifecycle-store-contracts.ncl`,
`nominal-evidence-identities.ncl`, `generated/cairn-policy.json`). Neither file
this change must vendor is named there, and neither carries a per-file notice:

| Path at R_C | Git blob |
|---|---|
| `contracts/assurance-targets.ncl` | `d9cb31625176e940aa4240774dab6fe22a7085ce` |
| `cairn-policy/assurance-targets.ncl` | `ee45346022a2956dcabd52fd7876fbba615b2b81` |

Octet's vendor record (`vendor/dependencies.ncl` at R_O) calls
`contracts/assurance-targets.ncl` "MIT OR Apache-2.0 (core and reusable
material)" and vendors it with `LICENSE-MIT`/`LICENSE-APACHE`, but that is
Octet's distribution statement, not a Cairn grant naming these paths, and it
does not cover `cairn-policy/assurance-targets.ncl` at all.

D-J says to treat the material as MIT OR Apache-2.0 and to stop and report if
Cairn's current licensing differs for these paths. Cairn's written boundary
does not name either path, and its catch-all assigns unmarked repository
material AGPL-3.0-or-later. The import was not performed.

## Resolution

Britton approved the grant added at Cairn
`830b5446bca69497d34ba65b22d8833927e3b5da`: `docs/licensing.md` now has
`| `contracts/**` and `cairn-policy/assurance-targets.ncl` |
`MIT OR Apache-2.0`, at every revision and for byte-for-byte copies |`, and
`cairn-policy/LICENSING.md` has the "Consumer contracts and assurance-targets
policy offer" section. WP8.3 imported the two files from that revision under
that grant; `cairn-policy/UPSTREAM.md` and `cairn-policy/LICENSING.md` record
the row, section, blob IDs and SHA-256 values.
