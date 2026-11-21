# Verus provider identity record for the bounds producer job. Version and
# toolchain labels are static; exact binary digests are recorded per run in the
# receipt's verifier identity.
{
  schema_version = "trellis-verus-provider-profile/v1",
  producer_id = "trellis.bounds.verus.mantle.v1",
  verus = {
    version = "0.2026.10.18.7d82994",
    derivation = "nix/verus.nix",
    binary = "terrane-verus",
  },
  z3 = {
    bundled_with_verus = true,
    binary = "z3",
  },
  rust_toolchain = {
    channel = "1.99.1",
    bundled_with_verus = true,
    manifest = "nix/verus.nix",
  },
  network_allowed = false,
  non_claims = [
    "The provider profile names pinned labels; it does not prove Z3 or Verus soundness",
    "Exact verifier and solver digests are recorded per run in the receipt, not in this static record",
  ],
}
