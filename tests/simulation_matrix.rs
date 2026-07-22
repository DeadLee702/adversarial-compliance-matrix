use std::process::Command;

struct TestCase {
    filename: &'static str,
    expected_hex: &'static str,
}

#[test]
fn test_adversarial_compliance_matrix() {
    // Ensure fixtures are generated
    generate_fixtures();

    let test_cases = vec![
        TestCase {
            filename: "test/incident_7f3a.evkp",
            expected_hex: "0x0F2E",
        },
        TestCase {
            filename: "test/incident_12b9.evkp",
            expected_hex: "0x0E1A",
        },
        TestCase {
            filename: "test/incident_3c8d.evkp",
            expected_hex: "0x0D44",
        },
        TestCase {
            filename: "test/incident_5e41.evkp",
            expected_hex: "0x1A4F",
        },
        TestCase {
            filename: "test/incident_6a2f.evkp",
            expected_hex: "0x1B88",
        },
        TestCase {
            filename: "test/incident_8b7c.evkp",
            expected_hex: "0x1C2B",
        },
        TestCase {
            filename: "test/incident_9d1e.evkp",
            expected_hex: "0x2A90",
        },
        TestCase {
            filename: "test/incident_a4f2.evkp",
            expected_hex: "0x2B11",
        },
        TestCase {
            filename: "test/incident_b5c3.evkp",
            expected_hex: "0x2C7F",
        },
        TestCase {
            filename: "test/incident_c6d9.evkp",
            expected_hex: "0x3A01",
        },
        TestCase {
            filename: "test/incident_d7e5.evkp",
            expected_hex: "0x3B99",
        },
        TestCase {
            filename: "test/incident_e8a1.evkp",
            expected_hex: "0x3C4D",
        },
    ];

    // Test all malicious incidents
    for test_case in test_cases {
        let output = Command::new("./target/release/evk")
            .arg("verify")
            .arg(test_case.filename)
            .arg("--cert")
            .output()
            .expect("Failed to execute evk command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!(
            "Testing {}: stdout='{}', stderr='{}'",
            test_case.filename, stdout, stderr
        );

        // Verify output contains "INVALID"
        assert!(
            stdout.contains("INVALID"),
            "Expected INVALID status for {}, got: {}",
            test_case.filename,
            stdout
        );

        // Verify output contains the specific hex code
        assert!(
            stdout.contains(test_case.expected_hex),
            "Expected hex code {} for {} in output: {}",
            test_case.expected_hex,
            test_case.filename,
            stdout
        );
    }

    // Test clean anchor file
    let clean_output = Command::new("./target/release/evk")
        .arg("verify")
        .arg("test/incident_clean.evkp")
        .arg("--cert")
        .output()
        .expect("Failed to execute evk command for clean file");

    let clean_stdout = String::from_utf8_lossy(&clean_output.stdout);
    println!("Testing clean file: stdout='{}'", clean_stdout);

    // Verify clean file returns VALID
    assert!(
        clean_stdout.contains("VALID"),
        "Expected VALID status for clean file, got: {}",
        clean_stdout
    );
}

fn generate_fixtures() {
    // Generate fixtures using gen_fixtures binary
    let output = Command::new("./target/release/gen_fixtures")
        .output()
        .expect("Failed to execute gen_fixtures command");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("gen_fixtures failed: {}", stderr);
        panic!("Failed to generate test fixtures");
    }

    println!("Fixtures generated successfully");
}
