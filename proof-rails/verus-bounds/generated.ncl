# Generated source record for the bounds convergence contract.
#
# Values come from tool output: `b3sum` over the reviewed specification and
# over the producer job's bound source paths. The JSON is exported with
#   nickel export --format json generated.ncl | jq -S . > generated.json
let Cairn = import "../../cairn-policy/assurance-targets-contract.ncl" in

let input = import "./generated-input.json" in

{
  schema_version = "cairn.assurance-target-sources.v1",
  specifications = input.specifications,
  entries = input.entries,
} | Cairn.GeneratedSources
