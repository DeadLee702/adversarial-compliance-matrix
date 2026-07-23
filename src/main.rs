mod cvm;

use cvm::{verify_file, Verdict};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: evk verify <path> [--cert] [--json]");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  evk verify test/incident_clean.evkp");
        eprintln!("  evk verify test/incident_7f3a.evkp --json");
        std::process::exit(1);
    }

    let command = &args[1];

    if command != "verify" {
        eprintln!("Unknown command: {}", command);
        std::process::exit(1);
    }

    if args.len() < 3 {
        eprintln!("Usage: evk verify <path> [--cert] [--json]");
        std::process::exit(1);
    }

    let file_path = &args[2];
    let use_json = args.iter().any(|a| a == "--json");

    match verify_file(file_path) {
        Ok(report) => {
            if use_json {
                println!("{}", report.to_json_pretty());
            } else {
                println!("{}", format_report(&report));
            }

            let exit_code = match report.verdict {
                Verdict::Pura => 0,
                Verdict::Vigla => 0,
                Verdict::Poluita => 1,
            };
            std::process::exit(exit_code);
        }
        Err(e) => {
            eprintln!("Error verifying file: {}", e);
            std::process::exit(2);
        }
    }
}

fn format_report(report: &cvm::ComplianceReport) -> String {
    let status_str = format!("0x{:04X}", report.status_code);
    let validity = if report.verdict == cvm::Verdict::Pura {
        "VALID"
    } else {
        "INVALID"
    };

    let mut output = format!("{}: {} [{}]\n", validity, report.message, status_str);
    output.push_str(&format!("  Verdict: {}\n", report.verdict.as_str()));
    output.push_str(&format!("  Incident: {}\n", report.incident_type));
    output.push_str(&format!("  Severity: {}\n", report.severity));
    output.push_str(&format!("  Confidence: {:.2}\n", report.confidence));
    output.push_str(&format!("  Action: {}\n", report.enforcement_action));
    output.push_str(&format!("  Timestamp: {}\n", report.timestamp));
    output
}
