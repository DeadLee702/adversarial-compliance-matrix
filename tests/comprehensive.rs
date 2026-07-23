use std::fs;
use std::path::PathBuf;
use std::process::Command;

// ─── Helpers ──────────────────────────────────────────────────────────

fn cargo_bin(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("release");
    path.push(name);
    path
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!("acm_test_{}_{}", prefix, std::process::id()));
        fs::create_dir_all(&path).unwrap();
        TempDir { path }
    }
    fn join(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }
    fn join_s(&self, name: String) -> PathBuf {
        self.path.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_incident(path: &PathBuf, code: u16, payload: &[u8]) {
    let mut data = code.to_be_bytes().to_vec();
    data.extend_from_slice(payload);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, &data).unwrap();
}

fn run_evk_verify(cwd: &PathBuf, file_arg: &str, cert: bool) -> (bool, String, String) {
    let mut cmd = Command::new(cargo_bin("evk"));
    cmd.current_dir(cwd).arg("verify").arg(file_arg);
    if cert {
        cmd.arg("--cert");
    }
    let output = cmd.output().expect("failed to execute evk");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn run_gen_fixtures(cwd: &PathBuf) -> bool {
    Command::new(cargo_bin("gen_fixtures"))
        .current_dir(cwd)
        .output()
        .expect("failed to execute gen_fixtures")
        .status
        .success()
}

// ─── All 12 malicious status codes ────────────────────────────────────

struct MaliciousCase {
    code: u16,
    hex: &'static str,
    message: &'static str,
}

fn all_malicious_cases() -> Vec<MaliciousCase> {
    vec![
        MaliciousCase {
            code: 0x0F2E,
            hex: "7f3a",
            message: "Handoff conflict detected",
        },
        MaliciousCase {
            code: 0x0E1A,
            hex: "12b9",
            message: "Race condition detected",
        },
        MaliciousCase {
            code: 0x0D44,
            hex: "3c8d",
            message: "Orphaned step detected",
        },
        MaliciousCase {
            code: 0x1A4F,
            hex: "5e41",
            message: "Transaction replay detected",
        },
        MaliciousCase {
            code: 0x1B88,
            hex: "6a2f",
            message: "Schema mutation detected",
        },
        MaliciousCase {
            code: 0x1C2B,
            hex: "8b7c",
            message: "Log truncation detected",
        },
        MaliciousCase {
            code: 0x2A90,
            hex: "9d1e",
            message: "Packet modification detected",
        },
        MaliciousCase {
            code: 0x2B11,
            hex: "a4f2",
            message: "Timestamp drift detected",
        },
        MaliciousCase {
            code: 0x2C7F,
            hex: "b5c3",
            message: "API spoofing detected",
        },
        MaliciousCase {
            code: 0x3A01,
            hex: "c6d9",
            message: "Prompt injection detected",
        },
        MaliciousCase {
            code: 0x3B99,
            hex: "d7e5",
            message: "Entropy leakage detected",
        },
        MaliciousCase {
            code: 0x3C4D,
            hex: "e8a1",
            message: "Register forgery detected",
        },
    ]
}

#[test]
fn test_all_malicious_incidents_via_gen_fixtures_and_evk() {
    let tmp = TempDir::new("all_malicious");
    assert!(run_gen_fixtures(&tmp.path), "gen_fixtures failed");

    for case in all_malicious_cases() {
        let filename = format!("test/incident_{}.evkp", case.hex);
        let (ok, stdout, _stderr) = run_evk_verify(&tmp.path, &filename, true);

        let is_critical_or_high = matches!(
            case.code,
            0x0F2E | 0x0E1A | 0x1A4F | 0x1C2B | 0x2A90 | 0x2C7F | 0x3A01 | 0x3B99 | 0x3C4D
        );
        if is_critical_or_high {
            assert!(
                !ok,
                "evk verify should exit non-zero for 0x{:04X} ({})",
                case.code, case.hex
            );
        }
        assert!(
            stdout.contains("INVALID"),
            "expected INVALID for {}: got {}",
            filename,
            stdout
        );
        assert!(
            stdout.contains(&format!("0x{:04X}", case.code)),
            "expected hex 0x{:04X} for {}: got {}",
            case.code,
            filename,
            stdout
        );
        assert!(
            stdout.contains(case.message),
            "expected message '{}' for {}: got {}",
            case.message,
            filename,
            stdout
        );
    }
}

#[test]
fn test_clean_incident_via_gen_fixtures_and_evk() {
    let tmp = TempDir::new("clean_incident");
    assert!(run_gen_fixtures(&tmp.path));

    let (ok, stdout, _stderr) = run_evk_verify(&tmp.path, "test/incident_clean.evkp", true);
    assert!(ok, "clean file should exit 0: {}", stdout);
    assert!(
        stdout.contains("VALID"),
        "expected VALID for clean file: got {}",
        stdout
    );
    assert!(
        stdout.contains("0x0000"),
        "expected 0x0000 for clean: got {}",
        stdout
    );
    assert!(
        stdout.contains("Clean artifact"),
        "expected 'Clean artifact' message: got {}",
        stdout
    );
}

// ─── Direct file tests (without gen_fixtures) ──────────────────────────

#[test]
fn test_each_malicious_code_directly() {
    let tmp = TempDir::new("direct_malicious");

    for case in all_malicious_cases() {
        let path = tmp.join_s(format!("incident_{:04x}.evkp", case.code));
        write_incident(&path, case.code, b"payload");

        let (ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
        let is_critical_or_high = matches!(
            case.code,
            0x0F2E | 0x0E1A | 0x1A4F | 0x1C2B | 0x2A90 | 0x2C7F | 0x3A01 | 0x3B99 | 0x3C4D
        );
        if is_critical_or_high {
            assert!(!ok, "should fail for 0x{:04X}", case.code);
        }
        assert!(
            stdout.contains("INVALID"),
            "should say INVALID for 0x{:04X}: {}",
            case.code,
            stdout
        );
        assert!(stdout.contains(&format!("0x{:04X}", case.code)));
        assert!(stdout.contains(case.message));
    }
}

#[test]
fn test_clean_file_directly() {
    let tmp = TempDir::new("direct_clean");
    let path = tmp.join("clean.evkp");
    write_incident(&path, 0x0000, b"clean data");

    let (ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
    assert!(ok);
    assert!(stdout.contains("VALID"));
    assert!(stdout.contains("0x0000"));
}

#[test]
fn test_unknown_malicious_code() {
    let tmp = TempDir::new("unknown_code");
    let path = tmp.join("unknown.evkp");
    write_incident(&path, 0xFFFF, b"unknown malicious");

    let (_ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
    assert!(stdout.contains("INVALID"));
    assert!(stdout.contains("0xFFFF"));
    assert!(stdout.contains("Unknown malicious status detected"));
}

#[test]
fn test_multiple_unknown_codes() {
    let tmp = TempDir::new("unknown_codes");

    for code in [0x0001u16, 0x1234, 0xABCD, 0xFFFE, 0x8000] {
        let path = tmp.join_s(format!("unknown_{:04x}.evkp", code));
        write_incident(&path, code, b"unknown");

        let (_ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
        assert!(stdout.contains("INVALID"));
        assert!(stdout.contains(&format!("0x{:04X}", code)));
    }
}

// ─── Edge cases ───────────────────────────────────────────────────────

#[test]
fn test_file_too_short_single_byte() {
    let tmp = TempDir::new("short_byte");
    let path = tmp.join("short.evkp");
    fs::write(&path, b"X").unwrap();

    let (_ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
    assert!(stdout.contains("INVALID"));
    assert!(stdout.contains("File too short"));
}

#[test]
fn test_empty_file() {
    let tmp = TempDir::new("empty_file");
    let path = tmp.join("empty.evkp");
    fs::write(&path, b"").unwrap();

    let (_ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
    assert!(stdout.contains("INVALID"));
    assert!(stdout.contains("File too short"));
}

#[test]
fn test_file_with_only_status_code_no_payload() {
    let tmp = TempDir::new("code_only");
    let path = tmp.join("code_only.evkp");
    write_incident(&path, 0x0F2E, b"");

    let (_ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
    assert!(stdout.contains("INVALID"));
    assert!(stdout.contains("0x0F2E"));
    assert!(stdout.contains("Handoff conflict detected"));
}

#[test]
fn test_clean_file_with_large_payload() {
    let tmp = TempDir::new("large_clean");
    let path = tmp.join("large.evkp");
    let payload = vec![0x41u8; 10000];
    write_incident(&path, 0x0000, &payload);

    let (ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
    assert!(ok);
    assert!(stdout.contains("VALID"));
}

#[test]
fn test_nonexistent_file_exits_with_error() {
    let tmp = TempDir::new("nonexistent");
    let (ok, _stdout, stderr) = run_evk_verify(&tmp.path, "/nonexistent/file.evkp", false);
    assert!(!ok);
    assert!(stderr.contains("Failed to read file") || stderr.contains("Error"));
}

// ─── Exit code verification ───────────────────────────────────────────

#[test]
fn test_exit_code_zero_for_clean() {
    let tmp = TempDir::new("exit_clean");
    let path = tmp.join("clean.evkp");
    write_incident(&path, 0x0000, b"clean");

    let output = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .arg("verify")
        .arg(path.to_str().unwrap())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0), "clean file should exit 0");
}

#[test]
fn test_exit_code_one_for_malicious() {
    let tmp = TempDir::new("exit_malicious");
    let path = tmp.join("malicious.evkp");
    write_incident(&path, 0x0F2E, b"malicious");

    let output = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .arg("verify")
        .arg(path.to_str().unwrap())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(1),
        "malicious file should exit 1"
    );
}

#[test]
fn test_exit_code_two_for_missing_file() {
    let tmp = TempDir::new("exit_missing");
    let output = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .arg("verify")
        .arg("/nonexistent/path.evkp")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "missing file should exit 2");
}

// ─── CLI argument tests ───────────────────────────────────────────────

#[test]
fn test_no_args_exits_nonzero() {
    let tmp = TempDir::new("no_args");
    let output = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage"));
}

#[test]
fn test_unknown_command_exits_nonzero() {
    let tmp = TempDir::new("bad_cmd");
    let output = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .arg("invalid_command")
        .arg("some_file")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown command"));
}

#[test]
fn test_verify_without_path_exits_nonzero() {
    let tmp = TempDir::new("no_path");
    let output = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .arg("verify")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage"));
}

// ─── gen_fixtures output verification ─────────────────────────────────

#[test]
fn test_gen_fixtures_creates_all_13_files() {
    let tmp = TempDir::new("gen_all_13");
    assert!(run_gen_fixtures(&tmp.path));

    let test_dir = tmp.join("test");
    assert!(test_dir.exists(), "test/ directory should be created");

    // 12 malicious + 1 clean = 13 files
    let mut evkp_count = 0;
    for entry in fs::read_dir(&test_dir).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".evkp") {
            evkp_count += 1;
        }
    }
    assert_eq!(
        evkp_count, 13,
        "should be 13 .evkp files (12 malicious + 1 clean)"
    );
}

#[test]
fn test_gen_fixtures_file_contents_have_correct_status_codes() {
    let tmp = TempDir::new("gen_contents");
    assert!(run_gen_fixtures(&tmp.path));

    for case in all_malicious_cases() {
        let path = tmp.join_s(format!("test/incident_{}.evkp", case.hex));
        assert!(path.exists(), "file should exist for {}", case.hex);

        let data = fs::read(&path).unwrap();
        assert!(data.len() >= 2, "file too short for {}", case.hex);
        let code = u16::from_be_bytes([data[0], data[1]]);
        assert_eq!(code, case.code, "status code mismatch for {}", case.hex);
    }

    let clean_path = tmp.join("test/incident_clean.evkp");
    let clean_data = fs::read(&clean_path).unwrap();
    let clean_code = u16::from_be_bytes([clean_data[0], clean_data[1]]);
    assert_eq!(clean_code, 0x0000, "clean file should have status 0x0000");
}

#[test]
fn test_gen_fixtures_malicious_files_contain_vulnerability_text() {
    let tmp = TempDir::new("gen_vuln_text");
    assert!(run_gen_fixtures(&tmp.path));

    let vulnerabilities = [
        ("7f3a", "Handoff conflict between microservices"),
        ("12b9", "Race condition in concurrent execution"),
        ("3c8d", "Orphaned workflow step detected"),
        ("5e41", "Transaction replay attack"),
        ("6a2f", "Unauthorized schema mutation"),
        ("8b7c", "Log truncation and tampering"),
        ("9d1e", "Network packet modification"),
        ("a4f2", "Timestamp synchronization drift"),
        ("b5c3", "API endpoint spoofing"),
        ("c6d9", "Prompt injection attack vector"),
        ("d7e5", "Entropy source leakage"),
        ("e8a1", "Register state forgery"),
    ];

    for (hex, vuln) in &vulnerabilities {
        let path = tmp.join_s(format!("test/incident_{}.evkp", hex));
        let data = fs::read(&path).unwrap();
        let payload = &data[2..]; // skip status code
        let text = std::str::from_utf8(payload).unwrap();
        assert!(
            text.contains(vuln),
            "file {} should contain '{}', got: {}",
            hex,
            vuln,
            text
        );
    }
}

#[test]
fn test_gen_fixtures_clean_file_contains_clean_text() {
    let tmp = TempDir::new("gen_clean_text");
    assert!(run_gen_fixtures(&tmp.path));

    let clean_path = tmp.join("test/incident_clean.evkp");
    let data = fs::read(&clean_path).unwrap();
    let payload = &data[2..];
    let text = std::str::from_utf8(payload).unwrap();
    assert!(
        text.contains("Clean artifact"),
        "clean file should contain 'Clean artifact': {}",
        text
    );
}

// ─── --cert flag behavior ─────────────────────────────────────────────

#[test]
fn test_cert_flag_does_not_change_exit_code() {
    let tmp = TempDir::new("cert_exit");

    let clean = tmp.join("clean.evkp");
    write_incident(&clean, 0x0000, b"clean");

    let mal = tmp.join("mal.evkp");
    write_incident(&mal, 0x0F2E, b"malicious");

    let clean_no_cert = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .args(["verify", clean.to_str().unwrap()])
        .output()
        .unwrap();
    let clean_cert = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .args(["verify", clean.to_str().unwrap(), "--cert"])
        .output()
        .unwrap();
    assert_eq!(clean_no_cert.status.code(), clean_cert.status.code());

    let mal_no_cert = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .args(["verify", mal.to_str().unwrap()])
        .output()
        .unwrap();
    let mal_cert = Command::new(cargo_bin("evk"))
        .current_dir(&tmp.path)
        .args(["verify", mal.to_str().unwrap(), "--cert"])
        .output()
        .unwrap();
    assert_eq!(mal_no_cert.status.code(), mal_cert.status.code());
}

// ─── Output format verification ───────────────────────────────────────

#[test]
fn test_output_format_valid() {
    let tmp = TempDir::new("fmt_valid");
    let path = tmp.join("clean.evkp");
    write_incident(&path, 0x0000, b"clean");

    let (ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
    assert!(ok);
    // Format: "VALID: <message> [0xXXXX]"
    assert!(stdout.starts_with("VALID:"));
    assert!(stdout.contains("[0x0000]"));
}

#[test]
fn test_output_format_invalid() {
    let tmp = TempDir::new("fmt_invalid");
    let path = tmp.join("mal.evkp");
    write_incident(&path, 0x1A4F, b"malicious");

    let (ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
    assert!(!ok);
    // Format: "INVALID: <message> [0xXXXX]"
    assert!(stdout.starts_with("INVALID:"));
    assert!(stdout.contains("[0x1A4F]"));
    assert!(stdout.contains("Transaction replay detected"));
}

// ─── Status code uniqueness (comprehensive) ───────────────────────────

#[test]
fn test_all_status_codes_are_unique() {
    let codes: Vec<u16> = all_malicious_cases().iter().map(|c| c.code).collect();
    let mut sorted = codes.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(codes.len(), sorted.len(), "all status codes must be unique");

    // Also verify none is 0x0000 (clean)
    for code in &codes {
        assert_ne!(*code, 0x0000, "malicious code should not be 0x0000");
    }
}

#[test]
fn test_all_malicious_codes_are_nonzero() {
    for case in all_malicious_cases() {
        assert_ne!(
            case.code, 0x0000,
            "malicious code 0x{:04X} should be non-zero",
            case.code
        );
    }
}

// ─── Batch verification ───────────────────────────────────────────────

#[test]
fn test_batch_verify_all_incidents() {
    let tmp = TempDir::new("batch");
    assert!(run_gen_fixtures(&tmp.path));

    let mut valid_count = 0;
    let mut invalid_count = 0;

    for entry in fs::read_dir(tmp.join("test")).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map(|e| e == "evkp").unwrap_or(false) {
            let (_ok, stdout, _stderr) = run_evk_verify(&tmp.path, path.to_str().unwrap(), false);
            if stdout.starts_with("VALID") {
                valid_count += 1;
            } else {
                invalid_count += 1;
                assert!(stdout.contains("INVALID"));
            }
        }
    }

    assert_eq!(valid_count, 1, "exactly 1 clean file should be valid");
    assert_eq!(
        invalid_count, 12,
        "exactly 12 malicious files should be invalid"
    );
}
