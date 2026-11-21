{ pkgs
, nickel
, currentVerus
, verusProofSource
, gatesScript
, jobContract
, profileSource
, profileJson
, providerSource
, providerJson
, producer
, envelope
, cargoToml
, cargoLock
, rustToolchainFile
, flakeLock
, sourceRevision
, rlimit
}:
pkgs.runCommand "trellis-bounds-verus-job" {
  nativeBuildInputs = [ currentVerus pkgs.bash pkgs.jq pkgs.b3sum pkgs.coreutils pkgs.diffutils producer ];
  preferLocalBuild = false;
  allowSubstitutes = true;
  stdoutMaxBytes = 1048576;
  stderrMaxBytes = 1048576;
} ''
  set -euo pipefail
  stage=prepare-inputs
  cap=${toString 1048576}

  # Nix removes failed output paths. Mirror original diagnostics into the build
  # log and keep a copy in TMPDIR for --keep-failed, without admitting an output.
  retain_failure() {
    status="$1"
    if [ "$status" -eq 0 ]; then
      return
    fi
    set +e
    printf 'Trellis bounds producer failed: stage=%s exit=%s\n' "$stage" "$status" >&2
    diagnostics="$TMPDIR/producer-diagnostics"
    mkdir -p "$diagnostics"
    printf 'stage=%s\nexit=%s\n' "$stage" "$status" > "$diagnostics/failure.txt"
    if [ -d "$out" ]; then
      cp -R "$out" "$diagnostics/output"
    fi
    for log in "$out"/artifacts/verus.stdout "$out"/artifacts/verus.stderr; do
      if [ -f "$log" ]; then
        printf '\n--- %s ---\n' "$log" >&2
        cat "$log" >&2
      fi
    done
    printf '\nOriginal producer diagnostics retained at %s (requires Nix --keep-failed).\n' "$diagnostics" >&2
    exit "$status"
  }
  trap 'retain_failure "$?"' EXIT

  export HOME="$TMPDIR/home"
  mkdir -p "$HOME" "$out/source" "$out/artifacts"

  # Verus reads only this filtered copy, never an ambient checkout.
  cp -R ${verusProofSource}/src/. "$out/source/src/"
  cp ${cargoToml} "$out/source/Cargo.toml"
  cp ${cargoLock} "$out/source/Cargo.lock"
  cp ${rustToolchainFile} "$out/source/rust-toolchain.toml"
  cp ${flakeLock} "$out/flake.lock"
  cp ${profileJson} "$out/profile.json"
  cp ${providerJson} "$out/provider-profile.json"

  printf '%s' ${pkgs.lib.escapeShellArg sourceRevision} \
    | grep -Eq '^[0-9a-f]{40}$' \
    || { printf 'source revision %s is not a 40-hex Git revision\n' ${pkgs.lib.escapeShellArg sourceRevision} >&2; exit 1; }
  printf '%s\n' ${pkgs.lib.escapeShellArg sourceRevision} > "$out/source-revision"

  stage=job-export
  ${nickel}/bin/nickel export --format json ${jobContract} > "$out/job.json"

  # Fail closed if committed runtime bytes drift from their Nickel source.
  stage=profile-freshness
  bash ${gatesScript} profile-freshness ${nickel}/bin/nickel ${profileSource} ${profileJson}
  stage=provider-profile-freshness
  bash ${gatesScript} export-freshness ${nickel}/bin/nickel ${providerSource} ${providerJson}

  stage=source-identities
  export B3SUM=${pkgs.b3sum}/bin/b3sum
  export JQ=${pkgs.jq}/bin/jq
  bash ${gatesScript} source-identities "$out" "$out/profile.json"

  stage=command-identity
  candidate_root=${currentVerus}/libexec/verus
  {
    printf 'terrane-verus\n'
    printf '%s\n' --triggers-mode silent --rlimit ${toString rlimit} --crate-type=lib
    printf '%s\n' --extern "vstd=$candidate_root/libvstd.rlib"
    printf '%s\n' --extern "verus_builtin=$candidate_root/libverus_builtin.rlib"
    printf '%s\n' --extern "verus_builtin_macros=$candidate_root/libverus_builtin_macros${pkgs.stdenv.hostPlatform.extensions.sharedLibrary}"
    printf '%s\n' --extern "verus_state_machines_macros=$candidate_root/libverus_state_machines_macros${pkgs.stdenv.hostPlatform.extensions.sharedLibrary}"
    printf '%s\n' -L "$candidate_root"
    printf '%s\n' --edition 2024
    printf '%s\n' "$out/source/src/lib.rs"
  } > "$out/artifacts/verus.command"

  stage=prove
  set +e
  terrane-verus \
    --triggers-mode silent \
    --rlimit ${toString rlimit} \
    --crate-type=lib \
    --extern vstd="$candidate_root/libvstd.rlib" \
    --extern verus_builtin="$candidate_root/libverus_builtin.rlib" \
    --extern verus_builtin_macros="$candidate_root/libverus_builtin_macros${pkgs.stdenv.hostPlatform.extensions.sharedLibrary}" \
    --extern verus_state_machines_macros="$candidate_root/libverus_state_machines_macros${pkgs.stdenv.hostPlatform.extensions.sharedLibrary}" \
    -L "$candidate_root" \
    --edition 2024 \
    "$out/source/src/lib.rs" \
    > "$TMPDIR/verus.stdout" 2> "$TMPDIR/verus.stderr"
  verus_status=$?
  set -e

  stdout_bytes=$(stat -c %s "$TMPDIR/verus.stdout")
  stderr_bytes=$(stat -c %s "$TMPDIR/verus.stderr")
  head -c "$cap" "$TMPDIR/verus.stdout" > "$out/artifacts/verus.stdout"
  head -c "$cap" "$TMPDIR/verus.stderr" > "$out/artifacts/verus.stderr"
  jq -n \
    --argjson stdout "$stdout_bytes" \
    --argjson stderr "$stderr_bytes" \
    --argjson cap "$cap" \
    '{stdout: {bytes: $stdout, truncated: ($stdout > $cap)}, stderr: {bytes: $stderr, truncated: ($stderr > $cap)}}' \
    > "$out/artifacts/streams.json"

  stage=result-check
  bash ${gatesScript} verus-result "$verus_status" "$out/artifacts/verus.stdout"

  stage=build-context
  jq -n \
    --argjson exit_code "$verus_status" \
    --arg package_lock "b3:$(b3sum --no-names "$out/source/Cargo.lock")" \
    --arg verifier_binary_path "$candidate_root/rust_verify" \
    --arg verifier_binary_digest "b3:$(b3sum --no-names "$candidate_root/rust_verify")" \
    --arg verifier_nix_closure ${pkgs.lib.escapeShellArg "${currentVerus}"} \
    --arg verifier_nix_closure_digest "b3:$(printf '%s' ${pkgs.lib.escapeShellArg "${currentVerus}"} | b3sum --no-names)" \
    --arg verifier_digest "b3:$(printf '%s' "$candidate_root" | b3sum --no-names)" \
    --arg toolchain_id "$(jq -r '.verus.toolchain | split(" ")[0]' "$candidate_root/version.json")" \
    --arg toolchain_digest "b3:$(b3sum --no-names "$candidate_root/version.json")" \
    --arg solver_digest "b3:$(b3sum --no-names "$candidate_root/z3")" \
    '{
      schema_version: "trellis-verus-build-context/v1",
      exit_code: $exit_code,
      package_lock_blake3: $package_lock,
      verifier: {
        verifier_id: "terrane-verus",
        verifier_digest: $verifier_digest,
        verifier_binary_path: $verifier_binary_path,
        verifier_binary_digest: $verifier_binary_digest,
        verifier_nix_closure: $verifier_nix_closure,
        verifier_nix_closure_digest: $verifier_nix_closure_digest,
        verifier_nix_closure_graph_digest: ""
      },
      toolchain: { toolchain_id: $toolchain_id, toolchain_digest: $toolchain_digest },
      solver: { solver_id: "z3", solver_digest: $solver_digest }
    }' > "$out/artifacts/build-context.json"

  stage=emit
  ${producer}/bin/trellis-verus-producer emit \
    --root "$out" \
    --envelope-tool ${envelope}/bin/trellis-verus-envelope

  stage=required-outputs
  bash ${gatesScript} required-outputs "$out" \
    job.json profile.json provider-profile.json source/src/lib.rs source/src/bounds.rs \
    source-revision flake.lock artifacts/verus.stdout artifacts/verus.stderr \
    artifacts/verus.command artifacts/streams.json artifacts/build-context.json \
    verus-receipt.json producer-report.json formal-proof-envelope.json \
    proof-cards.json evidence.json non-claims.json

  trap - EXIT
''
