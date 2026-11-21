#!/usr/bin/env bash
# Fail-closed gates for the bounds Verus producer job.
#
# Modes:
#   profile-freshness <nickel> <profile.ncl> <committed.json>
#   export-freshness  <nickel> <source.ncl> <committed.json>
#   verus-result      <exit-code> <stdout-file>
#   required-outputs  <root> <relative>...
#   source-identities <root> <profile.json>
#   self-test         <nickel> <b3sum> <jq>
set -euo pipefail

fail() {
  printf 'bounds-job-gate: %s\n' "$*" >&2
  return 1
}

reject() {
  printf 'bounds-job-gate: %s\n' "$*" >&2
  exit 1
}

profile_freshness() {
  local nickel="$1" source="$2" committed="$3" rendered
  rendered="$(mktemp)"
  "$nickel" export --format json "$source" > "$rendered"
  if ! cmp -s "$rendered" "$committed"; then
    rm -f "$rendered"
    fail "generated $(basename "$committed") is stale against $source; regenerate it"
    return 1
  fi
  rm -f "$rendered"
}

verus_result() {
  local code="$1" stdout="$2"
  if [ "$code" -ne 0 ]; then
    fail "verus exited $code; see artifacts/verus.stderr"
    return 1
  fi
  if [ ! -s "$stdout" ]; then
    fail "verus stdout is empty; no pass signal"
    return 1
  fi
  if ! grep -q '0 errors' "$stdout"; then
    fail "verus stdout lacks the '0 errors' pass signal"
    return 1
  fi
}

required_outputs() {
  local root="$1"
  shift
  local relative
  for relative in "$@"; do
    if [ ! -s "$root/$relative" ]; then
      fail "required output $relative is missing or empty"
      return 1
    fi
  done
}

source_identities() {
  local root="$1" profile="$2" count index path expected actual
  count="$("$JQ" --exit-status '.consumer.source.paths | length' "$profile")"
  if [ "$count" -le 0 ]; then
    fail "profile declares no source paths"
    return 1
  fi
  for ((index = 0; index < count; index++)); do
    path="$("$JQ" --raw-output --exit-status ".consumer.source.paths[$index].path" "$profile")"
    expected="$("$JQ" --raw-output --exit-status ".consumer.source.paths[$index].identity" "$profile")"
    if [ ! -f "$root/$path" ]; then
      fail "declared source $path is missing"
      return 1
    fi
    actual="b3:$("$B3SUM" --no-names "$root/$path")"
    if [ "$actual" != "$expected" ]; then
      fail "declared source identity for $path is $expected but bytes hash to $actual; regenerate profile.json"
      return 1
    fi
  done
}

self_test() {
  local nickel="$1" b3sum="$2" jq="$3" work
  work="$(mktemp -d)"
  trap 'rm -rf "${work:-}"' EXIT
  export B3SUM="$b3sum" JQ="$jq"

  printf '{ version = "one" }\n' > "$work/source.ncl"
  "$nickel" export --format json "$work/source.ncl" > "$work/generated.json"
  profile_freshness "$nickel" "$work/source.ncl" "$work/generated.json"
  printf '{ version = "two" }\n' > "$work/source.ncl"
  if profile_freshness "$nickel" "$work/source.ncl" "$work/generated.json" 2>/dev/null; then
    reject "stale generated profile was accepted"
  fi
  printf '{ version = "one" }\n' > "$work/source.ncl"

  printf 'verification results:: 3 verified, 0 errors\n' > "$work/pass.stdout"
  verus_result 0 "$work/pass.stdout"
  if verus_result 1 "$work/pass.stdout" 2>/dev/null; then
    reject "non-zero verus exit was accepted"
  fi
  printf 'verification results:: 0 verified, 1 errors\n' > "$work/fail.stdout"
  if verus_result 0 "$work/fail.stdout" 2>/dev/null; then
    reject "missing pass signal was accepted"
  fi
  : > "$work/empty.stdout"
  if verus_result 0 "$work/empty.stdout" 2>/dev/null; then
    reject "empty verus stdout was accepted"
  fi

  mkdir -p "$work/out/artifacts"
  printf 'proof\n' > "$work/out/artifacts/verus.stdout"
  required_outputs "$work/out" artifacts/verus.stdout
  if required_outputs "$work/out" artifacts/verus.stderr 2>/dev/null; then
    reject "missing required output was accepted"
  fi

  mkdir -p "$work/src"
  printf 'module x;\n' > "$work/src/lib.rs"
  printf '{"consumer":{"source":{"paths":[{"path":"src/lib.rs","identity":"b3:%s"}]}}}\n' \
    "$("$b3sum" --no-names "$work/src/lib.rs")" > "$work/profile.json"
  source_identities "$work" "$work/profile.json"
  printf 'module y;\n' > "$work/src/lib.rs"
  if source_identities "$work" "$work/profile.json" 2>/dev/null; then
    reject "source identity mismatch was accepted"
  fi
  printf '{"consumer":{"source":{"paths":[]}}}\n' > "$work/empty-profile.json"
  if source_identities "$work" "$work/empty-profile.json" 2>/dev/null; then
    reject "empty source path list was accepted"
  fi

  printf 'bounds-job-gate self-test: all positive and negative cases behaved as expected\n'
  trap - EXIT
  rm -rf "$work"
}

mode="${1:-}"
shift || true
case "$mode" in
  profile-freshness|export-freshness)
    [ "$#" -eq 3 ] || fail "$mode requires <nickel> <source.ncl> <committed.json>"
    profile_freshness "$1" "$2" "$3" || exit 1
    ;;
  verus-result)
    [ "$#" -eq 2 ] || fail "verus-result requires <exit-code> <stdout-file>"
    verus_result "$1" "$2" || exit 1
    ;;
  required-outputs)
    [ "$#" -ge 2 ] || fail "required-outputs requires <root> <relative>..."
    required_outputs "$@" || exit 1
    ;;
  source-identities)
    [ "$#" -eq 2 ] || fail "source-identities requires <root> <profile.json>"
    : "${B3SUM:?B3SUM must name the b3sum binary}"
    : "${JQ:?JQ must name the jq binary}"
    source_identities "$1" "$2" || exit 1
    ;;
  self-test)
    [ "$#" -eq 3 ] || fail "self-test requires <nickel> <b3sum> <jq>"
    self_test "$1" "$2" "$3" || exit 1
    ;;
  *)
    reject "unknown mode '${mode:-}'"
    ;;
esac
