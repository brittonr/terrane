# Octet requirement contract projected from the bounds assurance targets.
#
# The projection is Cairn's; export deterministic JSON with:
#   nickel export --format json contract.ncl | jq -S . > contract.json
let Cairn = import "../../cairn-policy/assurance-targets-contract.ncl" in

let targets = import "./targets.ncl" in
let generated = import "./generated.ncl" in

Cairn.project (std.array.first targets.contracts) generated
