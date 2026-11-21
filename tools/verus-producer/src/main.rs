//! Emit and verify the Trellis Verus producer evidence set.
//!
//! `emit` reads only the observed job output root and writes the receipt,
//! report, proof card deck, evidence bundle, and non-claims. `verify`
//! recomputes the same bytes and rejects any drift. Neither mode decides
//! theorem admission, realization, or release eligibility.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const RECEIPT_SCHEMA: &str = "octet-verus-proof-receipt/v1";
const REPORT_SCHEMA: &str = "trellis-verus-producer-report/v1";
const CARDS_SCHEMA: &str = "octet-proof-cards/v1";
const EVIDENCE_SCHEMA: &str = "octet-requirement-evidence/v1";
const ENVELOPE_SCHEMA: &str = "octet-formal-proof-envelope/v1";
const NON_CLAIMS_SCHEMA: &str = "trellis-verus-non-claims/v1";
const VERIFICATION_SCHEMA: &str = "trellis-verus-producer-verification/v1";
const JOB_SCHEMA: &str = "trellis-mantle-producer-job/v1";
const PROFILE_SCHEMA: &str = "trellis-verus-producer-profile/v1";
const PROVIDER_PROFILE_SCHEMA: &str = "trellis-verus-provider-profile/v1";
const BUILD_CONTEXT_SCHEMA: &str = "trellis-verus-build-context/v1";
const VERUS_PROVIDER_ID: &str = "verus-v1-compatibility";
const PASS_SIGNAL: &str = "0 errors";
const EXIT_SUCCESS: u32 = 0;

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("trellis-verus-producer: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let mode = args.first().map(String::as_str).unwrap_or_default();
    let mut root: Option<PathBuf> = None;
    let mut envelope_tool: Option<PathBuf> = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--root" => {
                root = Some(PathBuf::from(arg_value(&args, index, "--root")?));
                index += 2;
            }
            "--envelope-tool" => {
                envelope_tool = Some(PathBuf::from(arg_value(&args, index, "--envelope-tool")?));
                index += 2;
            }
            "--format" => {
                let format = arg_value(&args, index, "--format")?;
                if format != "json" {
                    return Err(format!("unsupported --format `{format}`"));
                }
                index += 2;
            }
            other => return Err(format!("unsupported argument `{other}`")),
        }
    }
    let root = root.ok_or("--root is required")?;
    match mode {
        "emit" => emit(&root, envelope_tool.as_deref()),
        "verify" => verify(&root),
        other => Err(format!("unsupported mode `{other}`; expected emit or verify")),
    }
}

fn arg_value(args: &[String], index: usize, flag: &str) -> Result<String, String> {
    args.get(index + 1)
        .filter(|value| !value.starts_with("--"))
        .cloned()
        .ok_or_else(|| format!("{flag} requires a value"))
}

// ---------------------------------------------------------------------------
// Shared byte and JSON helpers
// ---------------------------------------------------------------------------

fn read_bytes(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))
}

fn read_json(path: &Path) -> Result<serde_json::Value, String> {
    let bytes = read_bytes(path)?;
    serde_json::from_slice(&bytes).map_err(|error| format!("parse {}: {error}", path.display()))
}

fn write_json(path: &Path, value: &serde_json::Value) -> Result<(), String> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("serialize {}: {error}", path.display()))?;
    bytes.push(b'\n');
    fs::write(path, bytes).map_err(|error| format!("write {}: {error}", path.display()))
}

fn hash(bytes: &[u8]) -> String {
    format!("b3:{}", blake3::hash(bytes).to_hex())
}

fn hash_file(path: &Path) -> Result<String, String> {
    Ok(hash(&read_bytes(path)?))
}

fn text<'a>(value: &'a serde_json::Value, pointer: &str) -> Result<&'a str, String> {
    value
        .pointer(pointer)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| format!("missing string {pointer}"))
}

fn number(value: &serde_json::Value, pointer: &str) -> Result<u64, String> {
    value
        .pointer(pointer)
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| format!("missing integer {pointer}"))
}

fn flag(value: &serde_json::Value, pointer: &str) -> Result<bool, String> {
    value
        .pointer(pointer)
        .and_then(serde_json::Value::as_bool)
        .ok_or_else(|| format!("missing boolean {pointer}"))
}

fn string_array(value: &serde_json::Value, pointer: &str) -> Result<Vec<String>, String> {
    let rows = value
        .pointer(pointer)
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("missing array {pointer}"))?;
    rows.iter()
        .map(|row| {
            row.as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("{pointer} must contain strings"))
        })
        .collect()
}

fn dedup(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut ordered = Vec::new();
    for value in values {
        if seen.insert(value.clone()) {
            ordered.push(value);
        }
    }
    ordered
}

fn artifact(path: &str, blake3: String, bytes: u64, truncated: bool) -> serde_json::Value {
    serde_json::json!({
        "locator": path,
        "blake3": blake3,
        "bytes": bytes,
        "truncated": truncated,
    })
}

// ---------------------------------------------------------------------------
// Observed state
// ---------------------------------------------------------------------------

struct Observed {
    job: serde_json::Value,
    job_identity: String,
    profile: serde_json::Value,
    profile_identity: String,
    provider_identity: String,
    build: serde_json::Value,
    source_revision: String,
    policy_id: String,
    rail_index: u64,
    rail_id: String,
    source_bindings: Vec<(String, String)>,
    profile_source_paths: Vec<(String, String)>,
    stdout: serde_json::Value,
    stderr: serde_json::Value,
    command: serde_json::Value,
    pass_signal: String,
    non_claims: Vec<String>,
}

fn observe(root: &Path) -> Result<Observed, String> {
    let job_path = root.join("job.json");
    let profile_path = root.join("profile.json");
    let provider_path = root.join("provider-profile.json");
    let job_bytes = read_bytes(&job_path)?;
    let profile_bytes = read_bytes(&profile_path)?;
    let provider_bytes = read_bytes(&provider_path)?;
    let job = read_json(&job_path)?;
    let profile = read_json(&profile_path)?;
    let provider = read_json(&provider_path)?;

    if text(&job, "/schema_version")? != JOB_SCHEMA {
        return Err("job schema is not trellis-mantle-producer-job/v1".into());
    }
    if text(&job, "/owner")? != "Trellis" {
        return Err("job owner is not Trellis".into());
    }
    if text(&job, "/job_id")? != text(&profile, "/job_id")? {
        return Err("job id differs between the job contract and the profile".into());
    }
    if text(&profile, "/schema_version")? != PROFILE_SCHEMA {
        return Err("profile schema is not trellis-verus-producer-profile/v1".into());
    }
    if text(&provider, "/schema_version")? != PROVIDER_PROFILE_SCHEMA {
        return Err("provider profile schema is not trellis-verus-provider-profile/v1".into());
    }

    let profile_identity = hash(&profile_bytes);
    let provider_identity = hash(&provider_bytes);
    if text(&job, "/profile/identity")? != profile_identity {
        return Err(format!(
            "job profile identity {} differs from profile bytes {profile_identity}",
            text(&job, "/profile/identity")?
        ));
    }
    if text(&job, "/provider/identity")? != provider_identity {
        return Err(format!(
            "job provider identity {} differs from provider-profile bytes {provider_identity}",
            text(&job, "/provider/identity")?
        ));
    }
    if text(&profile, "/consumer/provider/profile_identity")? != provider_identity {
        return Err("profile provider identity differs from provider-profile bytes".into());
    }
    if flag(&job, "/source/network_allowed")? {
        return Err("job source network is allowed".into());
    }
    if flag(&job, "/execution/network_allowed")? {
        return Err("job execution network is allowed".into());
    }
    if flag(&provider, "/network_allowed")? {
        return Err("provider profile network is allowed".into());
    }
    if flag(&profile, "/extraction/network_allowed")? {
        return Err("profile extraction network is allowed".into());
    }

    let policy_id = text(&profile, "/policy_id")?.to_owned();
    let rail_index = number(&profile, "/rail_index")?;
    let rail_id = format!("{policy_id}#rail-{rail_index}");
    if text(&job, "/execution/rail_id")? != rail_id {
        return Err(format!(
            "job rail id {} differs from {rail_id}",
            text(&job, "/execution/rail_id")?
        ));
    }
    if text(&profile, "/claim/checked_scope")? != rail_id {
        return Err("profile claim scope differs from the rail id".into());
    }

    let mut profile_source_paths = Vec::new();
    let rows = profile
        .pointer("/consumer/source/paths")
        .and_then(serde_json::Value::as_array)
        .ok_or("missing profile source paths")?;
    let mut source_bindings = Vec::new();
    for row in rows {
        let path = text(row, "/path")?.to_owned();
        let identity = text(row, "/identity")?.to_owned();
        let actual = hash_file(&root.join(&path))?;
        if actual != identity {
            return Err(format!(
                "profile source {path} identity {identity} differs from bytes {actual}"
            ));
        }
        let repo_path = path
            .strip_prefix("source/")
            .ok_or_else(|| format!("profile source {path} is not under source/"))?
            .to_owned();
        source_bindings.push((repo_path, identity.clone()));
        profile_source_paths.push((path, identity));
    }
    if source_bindings.is_empty() {
        return Err("profile declares no source paths".into());
    }
    let declared = string_array(&job, "/source/paths")?;
    for (path, _) in &source_bindings {
        if !declared.iter().any(|entry| entry == path) {
            return Err(format!("source {path} is not declared by the job"));
        }
    }

    let build_path = root.join("artifacts/build-context.json");
    let build = read_json(&build_path)?;
    if text(&build, "/schema_version")? != BUILD_CONTEXT_SCHEMA {
        return Err("build context schema differs from trellis-verus-build-context/v1".into());
    }
    let exit_code = number(&build, "/exit_code")?;
    if exit_code != u64::from(EXIT_SUCCESS) {
        return Err(format!("verifier exited {exit_code}; no passing receipt admitted"));
    }

    let stdout_path = root.join("artifacts/verus.stdout");
    let stderr_path = root.join("artifacts/verus.stderr");
    let command_path = root.join("artifacts/verus.command");
    let stdout_bytes = read_bytes(&stdout_path)?;
    let stderr_bytes = read_bytes(&stderr_path)?;
    let command_bytes = read_bytes(&command_path)?;
    let pass_signal = String::from_utf8_lossy(&stdout_bytes).to_string();
    if !pass_signal.contains(PASS_SIGNAL) {
        return Err(format!(
            "verus stdout lacks the `{PASS_SIGNAL}` pass signal; refusing to admit a receipt"
        ));
    }

    let streams = read_json(&root.join("artifacts/streams.json"))?;
    let stdout = artifact(
        "artifacts/verus.stdout",
        hash(&stdout_bytes),
        number(&streams, "/stdout/bytes")?,
        flag(&streams, "/stdout/truncated")?,
    );
    let stderr = artifact(
        "artifacts/verus.stderr",
        hash(&stderr_bytes),
        number(&streams, "/stderr/bytes")?,
        flag(&streams, "/stderr/truncated")?,
    );
    let command = artifact(
        "artifacts/verus.command",
        hash(&command_bytes),
        command_bytes.len() as u64,
        false,
    );

    let source_revision = fs::read_to_string(root.join("source-revision"))
        .map_err(|error| format!("read source-revision: {error}"))?
        .trim()
        .to_owned();
    if source_revision.len() != 40 || !source_revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("source revision `{source_revision}` is not a 40-hex Git revision"));
    }

    let mut non_claims = Vec::new();
    for value in [
        string_array(&job, "/non_claims")?,
        string_array(&profile, "/non_claims")?,
        string_array(&provider, "/non_claims")?,
        vec![
            "The receipt records a passing verifier run over the listed source bytes; it is not theorem admission".to_owned(),
            "Mantle and Cairn perform their own bindings and judgment over these bytes".to_owned(),
        ],
    ] {
        non_claims.extend(value);
    }

    Ok(Observed {
        job,
        job_identity: hash(&job_bytes),
        profile,
        profile_identity,
        provider_identity,
        build,
        source_revision,
        policy_id,
        rail_index,
        rail_id,
        source_bindings,
        profile_source_paths,
        stdout,
        stderr,
        command,
        pass_signal: PASS_SIGNAL.to_owned(),
        non_claims: dedup(non_claims),
    })
}

// ---------------------------------------------------------------------------
// Expected evidence
// ---------------------------------------------------------------------------

struct Expected {
    receipt: serde_json::Value,
    report: serde_json::Value,
    evidence: serde_json::Value,
    non_claims: serde_json::Value,
    card: serde_json::Value,
}

fn expected(root: &Path) -> Result<Expected, String> {
    let observed = observe(root)?;
    let job_id = text(&observed.job, "/job_id")?.to_owned();
    let command_blake3 = text(&observed.command, "/blake3")?.to_owned();
    let policy_id = observed.policy_id.clone();
    let rail_index = observed.rail_index;
    let rail_id = observed.rail_id.clone();
    let profile_identity = observed.profile_identity.clone();
    let provider_identity = observed.provider_identity.clone();
    let source_revision = observed.source_revision.clone();
    let pass_signal = observed.pass_signal.clone();
    let non_claims = observed.non_claims.clone();
    let stdout = observed.stdout.clone();
    let stderr = observed.stderr.clone();
    let command = observed.command.clone();
    let source_bindings = observed.source_bindings.clone();
    let profile_source_paths = observed.profile_source_paths.clone();

    let receipt = serde_json::json!({
        "schema_version": RECEIPT_SCHEMA,
        "policy_id": policy_id,
        "rail_index": rail_index,
        "backend": "verus",
        "verifier_mode": "production",
        "producer_id": job_id,
        "checker_id": "trellis-verus-producer",
        "package_lock_blake3": text(&observed.build, "/package_lock_blake3")?,
        "verifier_identity": {
            "verifier_id": text(&observed.build, "/verifier/verifier_id")?,
            "verifier_digest": text(&observed.build, "/verifier/verifier_digest")?,
            "verifier_binary_path": text(&observed.build, "/verifier/verifier_binary_path")?,
            "verifier_binary_digest": text(&observed.build, "/verifier/verifier_binary_digest")?,
            "verifier_nix_closure": text(&observed.build, "/verifier/verifier_nix_closure")?,
            "verifier_nix_closure_digest": text(&observed.build, "/verifier/verifier_nix_closure_digest")?,
            "verifier_nix_closure_graph_digest": text(&observed.build, "/verifier/verifier_nix_closure_graph_digest")?,
        },
        "toolchain_identity": {
            "toolchain_id": text(&observed.build, "/toolchain/toolchain_id")?,
            "toolchain_digest": text(&observed.build, "/toolchain/toolchain_digest")?,
        },
        "solver_identity": {
            "solver_id": text(&observed.build, "/solver/solver_id")?,
            "solver_digest": text(&observed.build, "/solver/solver_digest")?,
        },
        "command_blake3": command_blake3,
        "source_bindings": source_bindings
            .iter()
            .map(|(path, blake3)| serde_json::json!({ "path": path, "blake3": blake3 }))
            .collect::<Vec<_>>(),
        "proof_bindings": [],
        "trust_boundaries": [
            "Verus and the bundled Z3 solver are trusted verifier components; the receipt records their identity, not their soundness",
            "The Rust/Verus semantics model of u64 arithmetic is trusted",
        ],
        "checked_scope": rail_id,
        "status": "passed",
        "pass_signal": pass_signal,
        "proof_artifact_identity": format!("artifacts/verus.command#{command_blake3}"),
        "stdout_artifact": stdout,
        "stderr_artifact": stderr,
        "non_claims": non_claims,
    });

    let artifacts = vec![
        serde_json::json!({
            "role": "verus.stdout",
            "locator": text(&stdout, "/locator")?,
            "blake3": text(&stdout, "/blake3")?,
            "bytes": number(&stdout, "/bytes")?,
            "truncated": flag(&stdout, "/truncated")?,
        }),
        serde_json::json!({
            "role": "verus.stderr",
            "locator": text(&stderr, "/locator")?,
            "blake3": text(&stderr, "/blake3")?,
            "bytes": number(&stderr, "/bytes")?,
            "truncated": flag(&stderr, "/truncated")?,
        }),
        serde_json::json!({
            "role": "verus.command",
            "locator": text(&command, "/locator")?,
            "blake3": text(&command, "/blake3")?,
            "bytes": number(&command, "/bytes")?,
            "truncated": flag(&command, "/truncated")?,
        }),
    ];

    let report = serde_json::json!({
        "schema_version": REPORT_SCHEMA,
        "owner": "Trellis",
        "job_id": job_id,
        "job_identity": observed.job_identity,
        "policy_id": observed.policy_id,
        "rail_index": rail_index,
        "rail_id": observed.rail_id,
        "status": "passed",
        "command_argv": command_argv(root)?,
        "command_blake3": text(&command, "/blake3")?,
        "exit_code": number(&observed.build, "/exit_code")?,
        "pass_signal": observed.pass_signal,
        "checked_scope": observed.rail_id,
        "source_revision": source_revision,
        "source_bindings": source_bindings
            .iter()
            .map(|(path, blake3)| serde_json::json!({ "path": path, "blake3": blake3 }))
            .collect::<Vec<_>>(),
        "profile_identity": profile_identity,
        "provider_profile_identity": provider_identity,
        "tool_identities": {
            "verifier": observed.build.pointer("/verifier").cloned().unwrap_or_default(),
            "toolchain": observed.build.pointer("/toolchain").cloned().unwrap_or_default(),
            "solver": observed.build.pointer("/solver").cloned().unwrap_or_default(),
        },
        "limits": observed.job.pointer("/limits").cloned().unwrap_or_default(),
        "artifacts": artifacts,
        "non_claims": observed.non_claims,
    });

    let card = serde_json::json!({
        "proof_or_property_id": rail_id,
        "source_requirement_or_policy_id": text(&observed.profile, "/claim/requirement_id")?,
        "policy_id": policy_id,
        "rail_index": rail_index,
        "checked_rust_source_scope": rail_id,
        "source_paths": profile_source_paths.iter().map(|(path, _)| path.clone()).collect::<Vec<_>>(),
        "proof_target_paths": ["source/src/bounds.rs"],
        "verifier_summary": observed.build.pointer("/verifier").cloned().unwrap_or_default(),
        "toolchain_summary": observed.build.pointer("/toolchain").cloned().unwrap_or_default(),
        "solver_summary": observed.build.pointer("/solver").cloned().unwrap_or_default(),
        "receipt_path": "verus-receipt.json",
        "artifact_blake3_identities": [
            { "artifact": text(&stdout, "/locator")?, "blake3": text(&stdout, "/blake3")? },
            { "artifact": text(&stderr, "/locator")?, "blake3": text(&stderr, "/blake3")? },
            { "artifact": text(&command, "/locator")?, "blake3": text(&command, "/blake3")? },
        ],
        "status": "passed",
        "evidence_tier": "formal_proof",
        "diagnostics": [],
        "non_claims": non_claims,
    });

    let evidence = serde_json::json!({
        "schema_version": EVIDENCE_SCHEMA,
        "contract_id": policy_id,
        "contract_blake3": serde_json::Value::Null,
        "source_snapshot": profile_source_paths
            .iter()
            .map(|(path, blake3)| serde_json::json!({ "locator": path, "blake3": blake3 }))
            .collect::<Vec<_>>(),
        "proof_decks": [{
            "evidence_id": rail_id,
            "deck": "proof-cards.json",
            "artifact_root": ".",
        }],
        "empirical_checks": [],
        "assumption_records": [],
        "non_claims": non_claims,
    });

    let non_claims_document = serde_json::json!({
        "schema_version": NON_CLAIMS_SCHEMA,
        "job_id": job_id,
        "non_claims": non_claims,
    });

    Ok(Expected {
        receipt,
        report,
        evidence,
        non_claims: non_claims_document,
        card,
    })
}

fn command_argv(root: &Path) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(root.join("artifacts/verus.command"))
        .map_err(|error| format!("read artifacts/verus.command: {error}"))?;
    Ok(text.lines().map(str::to_owned).collect())
}

// ---------------------------------------------------------------------------
// Envelope
// ---------------------------------------------------------------------------

fn adapt_envelope(root: &Path, tool: &Path) -> Result<serde_json::Value, String> {
    let receipt = root.join("verus-receipt.json");
    let output = root.join("formal-proof-envelope.json");
    let status = Command::new(tool)
        .arg("--receipt")
        .arg(&receipt)
        .arg("--output")
        .arg(&output)
        .status()
        .map_err(|error| format!("run {}: {error}", tool.display()))?;
    if !status.success() {
        return Err(format!("envelope tool exited with {status}"));
    }
    let envelope = read_json(&output)?;
    validate_envelope(&envelope)?;
    Ok(envelope)
}

fn validate_envelope(envelope: &serde_json::Value) -> Result<(), String> {
    if text(envelope, "/schema_version")? != ENVELOPE_SCHEMA {
        return Err("envelope schema is not octet-formal-proof-envelope/v1".into());
    }
    if text(envelope, "/provider_id")? != VERUS_PROVIDER_ID {
        return Err("envelope provider is not verus-v1-compatibility".into());
    }
    if text(envelope, "/status")? != "passed" {
        return Err("envelope status is not passed".into());
    }
    if text(envelope, "/evidence_tier")? != "formal_proof" {
        return Err("envelope evidence tier is not formal_proof".into());
    }
    let receipt = envelope
        .pointer("/provider/payload/receipt")
        .ok_or("envelope has no preserved Verus receipt payload")?;
    if text(receipt, "/schema_version")? != RECEIPT_SCHEMA {
        return Err("envelope payload receipt schema differs".into());
    }
    if text(receipt, "/status")? != "passed" {
        return Err("envelope payload receipt status is not passed".into());
    }
    Ok(())
}

fn cards_with_envelope(card: serde_json::Value, envelope: Option<&serde_json::Value>) -> serde_json::Value {
    let mut card = card;
    if let Some(envelope) = envelope {
        card["formal_proof_envelope"] = envelope.clone();
    }
    serde_json::json!({ "schema_version": CARDS_SCHEMA, "cards": [card] })
}

// ---------------------------------------------------------------------------
// Modes
// ---------------------------------------------------------------------------

fn emit(root: &Path, envelope_tool: Option<&Path>) -> Result<(), String> {
    let expected = expected(root)?;
    write_json(&root.join("verus-receipt.json"), &expected.receipt)?;
    let envelope = match envelope_tool {
        Some(tool) => Some(adapt_envelope(root, tool)?),
        None => None,
    };
    write_json(&root.join("producer-report.json"), &expected.report)?;
    write_json(&root.join("proof-cards.json"), &cards_with_envelope(expected.card, envelope.as_ref()))?;
    write_json(&root.join("evidence.json"), &expected.evidence)?;
    write_json(&root.join("non-claims.json"), &expected.non_claims)?;
    verify(root)
}

fn verify(root: &Path) -> Result<(), String> {
    let expected = expected(root)?;
    let mut checked = 0usize;
    let compare = |name: &str, recorded: serde_json::Value, wanted: &serde_json::Value| -> Result<(), String> {
        if recorded == *wanted {
            Ok(())
        } else {
            Err(format!("{name} differs from the recomputed bytes"))
        }
    };
    compare("verus-receipt.json", read_json(&root.join("verus-receipt.json"))?, &expected.receipt)?;
    checked += 1;
    compare("producer-report.json", read_json(&root.join("producer-report.json"))?, &expected.report)?;
    checked += 1;
    compare("evidence.json", read_json(&root.join("evidence.json"))?, &expected.evidence)?;
    checked += 1;
    compare("non-claims.json", read_json(&root.join("non-claims.json"))?, &expected.non_claims)?;
    checked += 1;

    let envelope = read_json(&root.join("formal-proof-envelope.json"))?;
    validate_envelope(&envelope)?;
    let expected_cards = cards_with_envelope(expected.card, Some(&envelope));
    compare("proof-cards.json", read_json(&root.join("proof-cards.json"))?, &expected_cards)?;
    checked += 1;

    let receipt_bytes = serde_json::to_vec(&expected.receipt).map_err(|error| error.to_string())?;
    let summary = serde_json::json!({
        "schema_version": VERIFICATION_SCHEMA,
        "status": "passed",
        "checked_documents": checked,
        "rail_id": text(&expected.receipt, "/checked_scope")?,
        "receipt_identity": hash(&receipt_bytes),
        "non_claims": [
            "Verification recomputes this producer's own documents; it is not theorem admission or external acceptance",
        ],
    });
    println!("{}", serde_json::to_string_pretty(&summary).map_err(|error| error.to_string())?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    struct Fixture {
        root: PathBuf,
    }

    impl Fixture {
        fn new(exit_code: u64) -> Self {
            let dir = tempfile::tempdir().expect("tempdir");
            let root = dir.keep();
            fs::create_dir_all(root.join("source/src")).expect("source dir");
            fs::create_dir_all(root.join("artifacts")).expect("artifacts dir");
            fs::write(root.join("source/src/lib.rs"), b"pub mod bounds;\n").expect("lib");
            fs::write(root.join("source/src/bounds.rs"), b"pub fn clamp() {}\n").expect("bounds");
            fs::write(root.join("flake.lock"), b"lock\n").expect("lock");
            fs::write(root.join("source-revision"), b"b9bd540cfd291b58bf2cb81a4a016b8763ccf9e2\n").expect("rev");
            fs::write(root.join("artifacts/verus.stdout"), b"--triggers-mode\nsilent\n").expect("stdout");
            fs::write(root.join("artifacts/verus.stderr"), b"").expect("stderr");
            fs::write(root.join("artifacts/verus.command"), b"terrane-verus\nfoo\n").expect("command");
            fs::write(
                root.join("artifacts/streams.json"),
                br#"{"stdout":{"bytes":23,"truncated":false},"stderr":{"bytes":0,"truncated":false}}"#,
            )
            .expect("streams");
            let provider = serde_json::json!({
                "schema_version": PROVIDER_PROFILE_SCHEMA,
                "producer_id": "trellis.bounds.verus.mantle.v1",
                "network_allowed": false,
                "non_claims": ["provider non-claim"],
            });
            write_json(&root.join("provider-profile.json"), &provider).expect("provider");
            let provider_identity = hash_file(&root.join("provider-profile.json")).expect("provider hash");
            let profile = serde_json::json!({
                "schema_version": PROFILE_SCHEMA,
                "owner": "Trellis",
                "job_id": "trellis.bounds.verus.mantle.v1",
                "policy_id": "trellis.bounds.verus",
                "rail_index": 0,
                "claim": { "requirement_id": "bounds.clamp", "checked_scope": "trellis.bounds.verus#rail-0" },
                "consumer": {
                    "source": { "paths": [
                        { "path": "source/src/lib.rs", "identity": hash_file(&root.join("source/src/lib.rs")).expect("h") },
                        { "path": "source/src/bounds.rs", "identity": hash_file(&root.join("source/src/bounds.rs")).expect("h") },
                    ] },
                    "provider": { "profile_identity": provider_identity },
                },
                "extraction": { "network_allowed": false },
                "non_claims": ["profile non-claim"],
            });
            write_json(&root.join("profile.json"), &profile).expect("profile");
            let profile_identity = hash_file(&root.join("profile.json")).expect("profile hash");
            let job = serde_json::json!({
                "schema_version": JOB_SCHEMA,
                "owner": "Trellis",
                "job_id": "trellis.bounds.verus.mantle.v1",
                "profile": { "path": "proof-rails/verus-bounds/generated/profile.json", "identity": profile_identity },
                "provider": { "path": "proof-rails/verus-bounds/generated/provider-profile.json", "identity": provider_identity },
                "source": { "root": "src", "paths": ["src/lib.rs", "src/bounds.rs"], "network_allowed": false },
                "execution": { "rail_id": "trellis.bounds.verus#rail-0", "network_allowed": false },
                "limits": { "cores": 8 },
                "non_claims": ["job non-claim"],
            });
            write_json(&root.join("job.json"), &job).expect("job");
            let build = serde_json::json!({
                "schema_version": BUILD_CONTEXT_SCHEMA,
                "exit_code": exit_code,
                "package_lock_blake3": "b3:lock",
                "verifier": {
                    "verifier_id": "terrane-verus",
                    "verifier_digest": "b3:aa",
                    "verifier_binary_path": "/nix/store/x/rust_verify",
                    "verifier_binary_digest": "b3:bb",
                    "verifier_nix_closure": "/nix/store/x",
                    "verifier_nix_closure_digest": "b3:cc",
                    "verifier_nix_closure_graph_digest": "",
                },
                "toolchain": { "toolchain_id": "nightly-2026-10-14", "toolchain_digest": "b3:dd" },
                "solver": { "solver_id": "z3", "solver_digest": "b3:ee" },
            });
            write_json(&root.join("artifacts/build-context.json"), &build).expect("build");
            Self { root }
        }

        fn path(&self, name: &str) -> PathBuf {
            self.root.join(name)
        }

        fn passing_stdout(&self) {
            fs::write(self.path("artifacts/verus.stdout"), b"verification results:: 3 verified, 0 errors\n")
                .expect("stdout");
            fs::write(
                self.path("artifacts/streams.json"),
                format!(
                    "{{\"stdout\":{{\"bytes\":{},\"truncated\":false}},\"stderr\":{{\"bytes\":0,\"truncated\":false}}}}",
                    b"verification results:: 3 verified, 0 errors\n".len()
                ),
            )
            .expect("streams");
        }

        fn envelope_script(&self, body: &str) -> PathBuf {
            let path = self.path("envelope-tool");
            fs::write(&path, body).expect("script");
            let mut permissions = fs::metadata(&path).expect("metadata").permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&path, permissions).expect("chmod");
            path
        }
    }

    const GOOD_ENVELOPE: &str = r#"#!/usr/bin/env bash
set -euo pipefail
while [ "$#" -gt 0 ]; do
  case "$1" in
    --receipt) receipt="$2"; shift 2 ;;
    --output) output="$2"; shift 2 ;;
    *) exit 2 ;;
  esac
done
status=$(jq -r .status "$receipt")
case "$status" in
  passed) ;;
  *) echo "unsupported Verus-v1 receipt status \`$status\`" >&2; exit 1 ;;
esac
jq -s '{schema_version:"octet-formal-proof-envelope/v1", provider_id:"verus-v1-compatibility", status:"passed", evidence_tier:"formal_proof", provider:{kind:"verus-v1", payload:{receipt:.[0]}}}' \
  "$receipt" > "$output"
"#;

    const WRONG_PROVIDER_ENVELOPE: &str = r#"#!/usr/bin/env bash
set -euo pipefail
while [ "$#" -gt 0 ]; do
  case "$1" in
    --receipt) receipt="$2"; shift 2 ;;
    --output) output="$2"; shift 2 ;;
    *) exit 2 ;;
  esac
done
jq -s '{schema_version:"octet-formal-proof-envelope/v1", provider_id:"not-verus", status:"passed", evidence_tier:"formal_proof", provider:{kind:"verus-v1", payload:{receipt:.[0]}}}' \
  "$receipt" > "$output"
"#;

    #[test]
    fn positive_emit_and_verify_round_trip() {
        let fixture = Fixture::new(0);
        fixture.passing_stdout();
        let tool = fixture.envelope_script(GOOD_ENVELOPE);
        emit(&fixture.root, Some(&tool)).expect("emit");
        verify(&fixture.root).expect("verify");
        let receipt = read_json(&fixture.path("verus-receipt.json")).expect("receipt");
        assert_eq!(text(&receipt, "/schema_version").unwrap(), RECEIPT_SCHEMA);
        assert_eq!(text(&receipt, "/status").unwrap(), "passed");
        assert_eq!(text(&receipt, "/checked_scope").unwrap(), "trellis.bounds.verus#rail-0");
        let cards = read_json(&fixture.path("proof-cards.json")).expect("cards");
        assert_eq!(text(&cards, "/cards/0/evidence_tier").unwrap(), "formal_proof");
        assert_eq!(
            text(&cards, "/cards/0/formal_proof_envelope/provider_id").unwrap(),
            VERUS_PROVIDER_ID
        );
    }

    #[test]
    fn failed_exit_emits_no_receipt() {
        let fixture = Fixture::new(1);
        fixture.passing_stdout();
        let tool = fixture.envelope_script(GOOD_ENVELOPE);
        assert!(emit(&fixture.root, Some(&tool)).is_err());
        assert!(!fixture.path("verus-receipt.json").exists());
    }

    #[test]
    fn missing_pass_signal_emits_no_receipt() {
        let fixture = Fixture::new(0);
        fs::write(fixture.path("artifacts/verus.stdout"), b"verification results:: 0 verified, 1 errors\n")
            .expect("stdout");
        let tool = fixture.envelope_script(GOOD_ENVELOPE);
        assert!(emit(&fixture.root, Some(&tool)).is_err());
        assert!(!fixture.path("verus-receipt.json").exists());
    }

    #[test]
    fn tampered_stdout_fails_verification() {
        let fixture = Fixture::new(0);
        fixture.passing_stdout();
        let tool = fixture.envelope_script(GOOD_ENVELOPE);
        emit(&fixture.root, Some(&tool)).expect("emit");
        fs::write(fixture.path("artifacts/verus.stdout"), b"verification results:: 3 verified, 0 errors\nforged")
            .expect("tamper");
        assert!(verify(&fixture.root).is_err());
    }

    #[test]
    fn unknown_receipt_status_fails_the_envelope_tool() {
        let fixture = Fixture::new(0);
        fixture.passing_stdout();
        let tool = fixture.envelope_script(GOOD_ENVELOPE);
        emit(&fixture.root, Some(&tool)).expect("emit");
        let mut receipt = read_json(&fixture.path("verus-receipt.json")).expect("receipt");
        receipt["status"] = serde_json::json!("unknown");
        write_json(&fixture.path("verus-receipt.json"), &receipt).expect("write");
        assert!(adapt_envelope(&fixture.root, &tool).is_err());
    }

    #[test]
    fn wrong_envelope_provider_is_rejected() {
        let fixture = Fixture::new(0);
        fixture.passing_stdout();
        let tool = fixture.envelope_script(WRONG_PROVIDER_ENVELOPE);
        assert!(emit(&fixture.root, Some(&tool)).is_err());
    }

    #[test]
    fn job_identity_mismatch_is_rejected() {
        let fixture = Fixture::new(0);
        fixture.passing_stdout();
        let mut job = read_json(&fixture.path("job.json")).expect("job");
        job["profile"]["identity"] = serde_json::json!("b3:wrong");
        write_json(&fixture.path("job.json"), &job).expect("write");
        assert!(expected(&fixture.root).is_err());
    }

    #[test]
    fn network_allowed_is_rejected() {
        let fixture = Fixture::new(0);
        fixture.passing_stdout();
        let mut job = read_json(&fixture.path("job.json")).expect("job");
        job["execution"]["network_allowed"] = serde_json::json!(true);
        write_json(&fixture.path("job.json"), &job).expect("write");
        assert!(expected(&fixture.root).is_err());
    }

    #[test]
    fn missing_artifact_is_rejected() {
        let fixture = Fixture::new(0);
        fixture.passing_stdout();
        fs::remove_file(fixture.path("artifacts/verus.command")).expect("remove");
        assert!(expected(&fixture.root).is_err());
    }
}
