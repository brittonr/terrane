//! Thin wrapper around Octet's provider-neutral Verus envelope adapter.
//!
//! The mapping is authored by octet-standards (`adapt_verus_v1_receipt`); this
//! binary only reads the receipt file, calls the adapter, and writes its bytes.
//! An unknown receipt status is rejected by the adapter and no output is
//! written.

// SPDX-License-Identifier: AGPL-3.0-or-later

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use octet_standards::formal_proof::adapt_verus_v1_receipt;
use octet_standards::proof::VerusProofReceipt;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("trellis-verus-envelope: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mut receipt_path: Option<PathBuf> = None;
    let mut output_path: Option<PathBuf> = None;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--receipt" => {
                receipt_path = Some(PathBuf::from(
                    args.next().ok_or("--receipt requires a path")?,
                ));
            }
            "--output" => {
                output_path = Some(PathBuf::from(
                    args.next().ok_or("--output requires a path")?,
                ));
            }
            other => return Err(format!("unsupported argument `{other}`")),
        }
    }
    let receipt_path = receipt_path.ok_or("--receipt is required")?;
    let output_path = output_path.ok_or("--output is required")?;
    let bytes = fs::read(&receipt_path)
        .map_err(|error| format!("read {}: {error}", receipt_path.display()))?;
    let receipt: VerusProofReceipt =
        serde_json::from_slice(&bytes).map_err(|error| format!("parse receipt: {error}"))?;
    let envelope = adapt_verus_v1_receipt(receipt)?;
    let mut rendered =
        serde_json::to_vec_pretty(&envelope).map_err(|error| format!("serialize envelope: {error}"))?;
    rendered.push(b'\n');
    fs::write(&output_path, rendered)
        .map_err(|error| format!("write {}: {error}", output_path.display()))
}
