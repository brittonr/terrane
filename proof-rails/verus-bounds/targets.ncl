# Assurance target for bounds.clamp over the frozen Trellis producer output.
#
# The registry row and the contract specification are derived from the
# content-bound registry export at revision
# 59fdf9c58cac89b79152a0e2b0fcd8300a654cc3; they are not copied by hand.
# Export deterministic JSON with:
#   nickel export --format json targets.ncl | jq -S . > targets.json
let Cairn = import "../../cairn-policy/assurance-targets-contract.ncl" in
let registry = import "./requirement-registry.json" in

let row = Cairn.requirement_from_registry registry "bounds.clamp" in
let spec = Cairn.specification_from_registry registry "bounds.clamp" in

{
  schema_version = "cairn.assurance-targets.v1",
  contracts = [
    {
      contract_id = "trellis.requirements.bounds",
      specification = spec,
      targets = [
        {
          requirement = row,
          assurance = "checked_proof",
          claim = "For the exact `src/lib.rs` and `src/bounds.rs` bytes bound by the job, Verus verifies that `bounds::clamp` returns `lo` when `value < lo`, `hi` when `value > hi`, and `value` otherwise, and that its result satisfies `in_range_spec(result, lo, hi)` whenever `lo <= hi`.",
          informal_remainder = "The requirement prose does not constrain behavior when `lo > hi`; the exec function requires `lo <= hi` and is not callable outside that precondition. The target does not prove that Rust callers respect the precondition, does not prove Verus or Z3 soundness, and does not prove that the `u64` model matches machine arithmetic beyond Verus's model. The checked Verus run covers all of `src/lib.rs`, which is larger than this bounds claim.",
          evidence_job = {
            job_id = "trellis.bounds.verus.mantle.v1",
            definition = "proof-rails/verus-bounds/mantle-job.ncl",
          },
          checked_scope = [
            "trellis.bounds.verus#rail-0",
          ],
          evidence_ids = [
            "trellis.bounds.verus#rail-0",
          ],
          evidence_classes = ["proof_card"],
          provider = "verus-v1-compatibility",
          negative_controls = [],
        },
      ],
      assumptions = [],
      exclusions = [],
      non_claims = [
        "evaluates supplied reviewed contracts and evidence only; does not authenticate reviewers or producers, prove verifier soundness, or establish whole-program correctness",
        "does not prove Rust caller preconditions",
        "does not prove Verus or Z3 soundness",
        "does not prove that the u64 model matches machine arithmetic beyond Verus's model",
        "does not establish release eligibility",
      ],
    },
  ],
} | Cairn.TargetsFile
