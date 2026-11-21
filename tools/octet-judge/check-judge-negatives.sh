#!/usr/bin/env bash
# Negative fixtures for the Octet requirements judgment.
#
# Usage: check-judge-negatives.sh <cargo-octet> <contract.json> <source-root> <work-dir>
#
# Every case must make `requirements check` exit non-zero.
set -euo pipefail
cargo_octet="$1"
contract="$2"
source_root="$3"
work="$4"
mkdir -p "$work"

check() {
  "$cargo_octet" requirements check \
    --contract "$1" --evidence "$2" --source-root "$source_root" \
    --output "$work/last-decision.json" --output-format json > "$work/last-check.txt" 2>&1
}

expect_rejected() {
  if check "$1" "$2"; then
    printf 'negative case was accepted: %s\n' "$3" >&2
    exit 1
  fi
  printf 'refused: %s\n' "$3"
}

evidence="$source_root/evidence.json"

printf 'positive control\n'
check "$contract" "$evidence"
jq --exit-status '.aggregate_result == "passed"' "$work/last-decision.json" > /dev/null

# 1. Wrong contract byte: point the clause at an evidence id it may not accept.
jq '(.clauses[0].allowed_evidence_ids) = ["trellis.bounds.verus#other"]' "$contract" > "$work/contract-wrong-byte.json"
expect_rejected "$work/contract-wrong-byte.json" "$evidence" "wrong contract byte"

# 2. Missing proof card: keep a deck reference whose file has no matching card.
jq '.proof_decks[0].evidence_id = "trellis.bounds.verus#missing"' "$evidence" > "$work/evidence-missing-card.json"
expect_rejected "$contract" "$work/evidence-missing-card.json" "missing card"

# 3. Provider mismatch: require a provider the envelope does not name.
jq '(.clauses[0].required_provider_id) = "aeneas-lean"' "$contract" > "$work/contract-provider.json"
expect_rejected "$work/contract-provider.json" "$evidence" "provider mismatch"

# 4. Empirical evidence for a formal clause: replace the deck with an empirical check.
jq '{schema_version, contract_id, contract_blake3, source_snapshot, proof_decks: [],
     empirical_checks: [{evidence_id: "trellis.bounds.verus#rail-0",
                         evidence_class: "test_run", status: "passed",
                         checker_id: "wrapper", command_argv: [],
                         checked_scope: ["trellis.bounds.verus#rail-0"],
                         source_bindings: [], artifact: null, artifact_blake3: null,
                         diagnostics: [], non_claims: ["empirical only"]}],
     assumption_records: [], non_claims}' "$evidence" > "$work/evidence-empirical.json"
expect_rejected "$contract" "$work/evidence-empirical.json" "empirical evidence for formal clause"
