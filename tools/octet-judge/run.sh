#!/usr/bin/env bash
# Run Octet's reviewed-requirement judgment over a realized producer output.
#
# Usage: run.sh <cargo-octet> <contract.json> <source-root> <output-dir> [evidence.json]
#
# The evidence bundle defaults to `<source-root>/evidence.json`, the deck
# `proof-cards.json` and the artifacts it names are resolved under
# `<source-root>`. The decision is Octet's; this wrapper only invokes it and
# requires the recorded aggregate to be `passed`.
set -euo pipefail
cargo_octet="$1"
contract="$2"
source_root="$3"
out="$4"
evidence="${5:-$source_root/evidence.json}"
mkdir -p "$out"

"$cargo_octet" requirements check \
  --contract "$contract" \
  --evidence "$evidence" \
  --source-root "$source_root" \
  --output "$out/decision.json" \
  --output-format json

"$cargo_octet" requirements verify \
  --contract "$contract" \
  --evidence "$evidence" \
  --decision "$out/decision.json" \
  --source-root "$source_root" \
  --output-format json > "$out/verification.json"

jq --exit-status '.aggregate_result == "passed"' "$out/decision.json" > /dev/null
jq --exit-status '.status == "passed" and .aggregate_result == "passed"' "$out/verification.json" > /dev/null
printf 'judgment passed: %s\n' "$out/decision.json"
