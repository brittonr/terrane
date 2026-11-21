# Trellis producer profile for the bounds Verus job. Static contract: the
# source identities bind the exact bytes the job proves, so a source edit makes
# the profile stale and the derivation fails closed until it is regenerated.
{
  schema_version = "trellis-verus-producer-profile/v1",
  owner = "Trellis",
  job_id = "trellis.bounds.verus.mantle.v1",
  policy_id = "trellis.bounds.verus",
  rail_index = 0,
  claim = {
    requirement_id = "bounds.clamp",
    companion_requirement_ids = ["bounds.in-range"],
    checked_scope = "trellis.bounds.verus#rail-0",
    module_scope = "bounds",
    crate_root = "source/src/lib.rs",
  },
  consumer = {
    source = {
      root = "source",
      paths = [
        {
          path = "source/src/lib.rs",
          identity = "b3:4b297b4ad379c7bd58fbb5a82826660e3292cfdfcef7a20270303d702f0d1676",
        },
        {
          path = "source/src/bounds.rs",
          identity = "b3:c81448f07814bbbdd99b70b8e40df6c3d1cafe8ecfbda8a4d57f6609e2ffe561",
        },
      ],
    },
    provider = {
      profile_identity = "b3:ff95cfcf82c969e36b960e9cfb193e24dd593c1525ed4e4e47a0046316bf81f1",
    },
  },
  provider_tools = {
    verus = { version = "0.2026.10.18.7d82994", binary = "terrane-verus" },
    z3 = { bundled_with_verus = true, binary = "z3" },
    rust_toolchain = { channel = "1.99.1", bundled_with_verus = true },
  },
  extraction = {
    backend = "verus",
    crate_root = "source/src/lib.rs",
    command_args = [
      "--triggers-mode",
      "silent",
      "--rlimit",
      "20",
      "--crate-type=lib",
      "--edition",
      "2024",
      "source/src/lib.rs",
    ],
    network_allowed = false,
  },
  artifacts = [
    { role = "verus.stdout", locator = "artifacts/verus.stdout" },
    { role = "verus.stderr", locator = "artifacts/verus.stderr" },
    { role = "verus.command", locator = "artifacts/verus.command" },
  ],
  declared_outputs = [
    "artifacts",
    "job.json",
    "source",
    "source-revision",
    "flake.lock",
    "profile.json",
    "provider-profile.json",
    "producer-report.json",
    "verus-receipt.json",
    "formal-proof-envelope.json",
    "proof-cards.json",
    "evidence.json",
    "non-claims.json",
  ],
  verus = { receipt_path = "verus-receipt.json" },
  non_claims = [
    "Not whole-crate correctness: the claim is scoped to the bounds module and the declared rail",
    "Not verifier or solver soundness",
    "Not theorem admission or release eligibility",
    "The checked run covers all of source/src/lib.rs, which is larger than the scoped claim",
  ],
}
