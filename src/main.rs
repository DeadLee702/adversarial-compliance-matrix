mod cvm;

use cvm::VerificationResult;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: evk verify <path> [--cert]");
        std::process::exit(1);
    }

    let command = &args[1];

    if command != "verify" {
        eprintln!("Unknown command: {}", command);
        std::process::exit(1);
    }

    if args.len() < 3 {
        eprintln!("Usage: evk verify <path> [--cert]");
        std::process::exit(1);
    }

    let file_path = &args[2];
    let _use_cert = args.len() > 3 && args[3] == "--cert";

    match verify_file(file_path) {
        Ok(result) => {
            println!("{}", result.format_output());
            std::process::exit(if result.is_valid { 0 } else { 1 });
        }
        Err(e) => {
            eprintln!("Error verifying file: {}", e);
            std::process::exit(2);
        }
    }
}

fn verify_file(path: &str) -> Result<VerificationResult, String> {
    let file_data = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;

    if file_data.len() < 2 {
        return Ok(VerificationResult::new(
            0x0000,
            false,
            "File too short to contain status code",
        ));
    }

    // Read status code (big-endian u16 from first 2 bytes)
    let status_code = u16::from_be_bytes([file_data[0], file_data[1]]);

    // Check if status code is 0x0000 (clean)
    if status_code == 0x0000 {
        return Ok(VerificationResult::new(status_code, true, "Clean artifact"));
    }

    // Any non-zero status code is invalid
    let message = match status_code {
        cvm::STATUS_HANDOFF_CONFLICT => "Handoff conflict detected",
        cvm::STATUS_RACE_CONDITION => "Race condition detected",
        cvm::STATUS_ORPHANED_STEP => "Orphaned step detected",
        cvm::STATUS_TRANSACTION_REPLAY => "Transaction replay detected",
        cvm::STATUS_SCHEMA_MUTATION => "Schema mutation detected",
        cvm::STATUS_LOG_TRUNCATION => "Log truncation detected",
        cvm::STATUS_PACKET_MODIFICATION => "Packet modification detected",
        cvm::STATUS_TIMESTAMP_DRIFT => "Timestamp drift detected",
        cvm::STATUS_API_SPOOFING => "API spoofing detected",
        cvm::STATUS_PROMPT_INJECTION => "Prompt injection detected",
        cvm::STATUS_ENTROPY_LEAKAGE => "Entropy leakage detected",
        cvm::STATUS_REGISTER_FORGERY => "Register forgery detected",
        _ => "Unknown malicious status detected",
    };

    Ok(VerificationResult::new(status_code, false, message))
}
