#!/usr/bin/env bash
# Negative fixtures for the exported requirement registry.
#
# Usage: check-registry-negatives.sh <cairn-convergence-binary> [root]
set -euo pipefail
cairn="$1"
root="${2:-.}"
registry="$root/proof-rails/verus-bounds/requirement-registry.json"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

verify() {
  "$cairn" requirement-registry verify --root "$root" --registry "$1" \
    --repository-id onixresearch/terrane > "$work/last.json" 2>&1
}

expect_rejected() {
  if verify "$1"; then
    printf 'negative case was accepted: %s\n' "$2" >&2
    exit 1
  fi
  printf 'refused: %s\n' "$2"
}

verify "$registry"
printf 'accepted: exported registry\n'

jq '.revision.value = "1111111111111111111111111111111111111111"' \
  "$registry" > "$work/moved-revision.json"
expect_rejected "$work/moved-revision.json" "moved revision"

jq '(.rows[] | select(.requirement_id == "bounds.clamp") | .specification_blake3) = ("0" * 64)' \
  "$registry" > "$work/edited-spec.json"
expect_rejected "$work/edited-spec.json" "edited specification"

jq '(.rows[] | select(.requirement_id == "bounds.clamp") | .row_blake3) = ("0" * 64)' \
  "$registry" > "$work/stale-row.json"
expect_rejected "$work/stale-row.json" "stale row"
