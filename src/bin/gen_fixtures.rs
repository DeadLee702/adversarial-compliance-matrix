use std::fs;
use std::io::Write;
use std::path::Path;

// Status codes matching those in src/cvm.rs
const STATUS_HANDOFF_CONFLICT: u16 = 0x0F2E;
const STATUS_RACE_CONDITION: u16 = 0x0E1A;
const STATUS_ORPHANED_STEP: u16 = 0x0D44;
const STATUS_TRANSACTION_REPLAY: u16 = 0x1A4F;
const STATUS_SCHEMA_MUTATION: u16 = 0x1B88;
const STATUS_LOG_TRUNCATION: u16 = 0x1C2B;
const STATUS_PACKET_MODIFICATION: u16 = 0x2A90;
const STATUS_TIMESTAMP_DRIFT: u16 = 0x2B11;
const STATUS_API_SPOOFING: u16 = 0x2C7F;
const STATUS_PROMPT_INJECTION: u16 = 0x3A01;
const STATUS_ENTROPY_LEAKAGE: u16 = 0x3B99;
const STATUS_REGISTER_FORGERY: u16 = 0x3C4D;

struct Incident {
    code: u16,
    hex_id: &'static str,
    vulnerability: &'static str,
}

fn main() {
    let incidents = vec![
        Incident {
            code: STATUS_HANDOFF_CONFLICT,
            hex_id: "7f3a",
            vulnerability: "Handoff conflict between microservices",
        },
        Incident {
            code: STATUS_RACE_CONDITION,
            hex_id: "12b9",
            vulnerability: "Race condition in concurrent execution",
        },
        Incident {
            code: STATUS_ORPHANED_STEP,
            hex_id: "3c8d",
            vulnerability: "Orphaned workflow step detected",
        },
        Incident {
            code: STATUS_TRANSACTION_REPLAY,
            hex_id: "5e41",
            vulnerability: "Transaction replay attack",
        },
        Incident {
            code: STATUS_SCHEMA_MUTATION,
            hex_id: "6a2f",
            vulnerability: "Unauthorized schema mutation",
        },
        Incident {
            code: STATUS_LOG_TRUNCATION,
            hex_id: "8b7c",
            vulnerability: "Log truncation and tampering",
        },
        Incident {
            code: STATUS_PACKET_MODIFICATION,
            hex_id: "9d1e",
            vulnerability: "Network packet modification",
        },
        Incident {
            code: STATUS_TIMESTAMP_DRIFT,
            hex_id: "a4f2",
            vulnerability: "Timestamp synchronization drift",
        },
        Incident {
            code: STATUS_API_SPOOFING,
            hex_id: "b5c3",
            vulnerability: "API endpoint spoofing",
        },
        Incident {
            code: STATUS_PROMPT_INJECTION,
            hex_id: "c6d9",
            vulnerability: "Prompt injection attack vector",
        },
        Incident {
            code: STATUS_ENTROPY_LEAKAGE,
            hex_id: "d7e5",
            vulnerability: "Entropy source leakage",
        },
        Incident {
            code: STATUS_REGISTER_FORGERY,
            hex_id: "e8a1",
            vulnerability: "Register state forgery",
        },
    ];

    // Create test directory
    let test_dir = Path::new("test");
    if !test_dir.exists() {
        fs::create_dir(test_dir).expect("Failed to create test directory");
    }

    // Generate malicious incidents
    for incident in &incidents {
        let filename = format!("test/incident_{}.evkp", incident.hex_id);
        generate_incident_file(&filename, incident.code, incident.vulnerability);
        println!("Generated: {}", filename);
    }

    // Generate clean anchor file
    let clean_filename = "test/incident_clean.evkp";
    generate_clean_file(clean_filename);
    println!("Generated: {}", clean_filename);
}

fn generate_incident_file(filename: &str, status_code: u16, vulnerability: &str) {
    let mut file = fs::File::create(filename).expect("Failed to create incident file");

    // Write status code as big-endian u16
    let bytes = status_code.to_be_bytes();
    file.write_all(&bytes).expect("Failed to write status code");

    // Write vulnerability payload
    file.write_all(vulnerability.as_bytes())
        .expect("Failed to write vulnerability text");
}

fn generate_clean_file(filename: &str) {
    let mut file = fs::File::create(filename).expect("Failed to create clean file");

    // Write clean status code (0x0000) as big-endian u16
    let bytes = (0u16).to_be_bytes();
    file.write_all(&bytes)
        .expect("Failed to write clean status code");

    // Write clean payload
    file.write_all(b"Clean artifact - no vulnerabilities")
        .expect("Failed to write clean payload");
}
