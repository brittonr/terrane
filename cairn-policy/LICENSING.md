# Cairn policy license grant for Terrane

The Cairn copyright holders offer the following exact Cairn policy files from
commit `b6759012db4dab0158936e995b7bad39ac10465b` under a choice of
`AGPL-3.0-or-later OR MIT OR Apache-2.0`. Cairn recorded the explicit grant
covering these historical Git blobs in commit
`d2c1dd7867350b787cfffe0e311fd20115238339`. This notice provides the
same grant with the copies, without requiring access to Cairn's Git origin.

| Cairn file in `cairn-policy/` | Copy in Terrane's `cairn-policy/` | Git blob |
|---|---|---|
| `default.ncl` | `upstream-source/default.ncl` | `0d04be0a1d3fbe3fa94157aab68cd75c7e3dccb6` |
| `contracts.ncl` | `upstream-source/contracts.ncl` | `293e587cff35e80c6b089cb5c4a2b12d6996a621` |
| `lifecycle-store-contracts.ncl` | `upstream-source/lifecycle-store-contracts.ncl` | `ea7becf0212d09af78e6c9f777d60c2e2a0086de` |
| `nominal-evidence-identities.ncl` | `upstream-source/nominal-evidence-identities.ncl` | `ba2c825f194bc82759d0eae916bb31912d435bae` |
| `generated/cairn-policy.json` | `generated/cairn-policy.base.json` | `57d1f8194930ceacde797af4b53f90ad1daeac18` |

Recipients may choose any one of the listed licenses for these copies.
Terrane distributes them under **MIT**; its complete terms, including the
Cairn contributors' copyright notice, are reproduced at
[`../LICENSES/Cairn-MIT.txt`](../LICENSES/Cairn-MIT.txt). The previous AGPL
option and its [license text](../LICENSES/AGPL-3.0-or-later.txt) remain
available. The Apache-2.0 option is available under its
[complete upstream terms](https://www.apache.org/licenses/LICENSE-2.0).

This grant covers only the named Cairn source and generated content. It does
not change the `MPL-2.0` license of Terrane's own `default.ncl`,
`contracts.ncl`, or other project source; it does not relicense Cairn's CLI,
exporter/toolchain, other Cairn files, or independently licensed third-party
material. Preserve notices and satisfy the terms of the license you choose.

## Assurance-targets material at Cairn 830b544

Cairn commit `830b5446bca69497d34ba65b22d8833927e3b5da` offers the
consumer-facing typed Nickel contracts under `contracts/**` and
`cairn-policy/assurance-targets.ncl` under `MIT OR Apache-2.0`, at every
revision and for byte-for-byte copies. Cairn records the offer in
`docs/licensing.md` (row `| `contracts/**` and
`cairn-policy/assurance-targets.ncl` | `MIT OR Apache-2.0`, at every revision
and for byte-for-byte copies |`) and in `cairn-policy/LICENSING.md`
("Consumer contracts and assurance-targets policy offer").

Terrane distributes these copies under MIT or Apache-2.0, choosing per use.
The complete texts are at [`../LICENSES/MIT.txt`](../LICENSES/MIT.txt) and
[`../LICENSES/Apache-2.0.txt`](../LICENSES/Apache-2.0.txt).

| Cairn file at `830b544` | Copy in Terrane's `cairn-policy/` | Git blob | SHA-256 |
|---|---|---|---|
| `contracts/assurance-targets.ncl` | `assurance-targets-contract.ncl` | `d9cb31625176e940aa4240774dab6fe22a7085ce` | `08bb2f6aee2f5336585cb8fb7224c601fed1bbe1b5cf5f4a264e57162b3318ab` |
| `cairn-policy/assurance-targets.ncl` | `assurance-targets-policy.ncl` | `ee45346022a2956dcabd52fd7876fbba615b2b81` | `54c2eb0a43d6ab46301a29cfe207b1ed687d742f5d5f2e1761475f69befb24f2` |

This offer covers only those Cairn paths and their copies. It does not
relicense Cairn's CLI, exporter or toolchain, the historical `b6759012`
material, or Terrane's own Nickel overlay. The earlier grant above remains
valid and its copies are unchanged.
