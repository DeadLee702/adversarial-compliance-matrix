# Adversarial Compliance Matrix

**A Rust CLI tool for simulating and detecting 12 real-world compliance and adversarial incidents.**

## 🎯 What is it?

**Adversarial Compliance Matrix** lets you generate and verify "compliance artifacts" (`.evkp` files) that represent different types of security, operational, or adversarial incidents.

It’s a lightweight training, testing, and red-teaming tool.

## ✨ Features

- Fast Rust-based verification engine
- 12 realistic adversarial/compliance incident types
- Fixture generator for easy testing
- Clean command-line interface (`evk verify`)

## 📋 The 12-Incident Matrix

| Incident                  | Code     | Description                          |
|---------------------------|----------|--------------------------------------|
| Handoff Conflict          | `0x0F2E` | Step executed by wrong actor        |
| Race Condition            | `0x0E1A` | Concurrent modification             |
| Orphaned Step             | `0x0D44` | Step with no parent process         |
| Transaction Replay        | `0x1A4F` | Re-execution of prior transaction   |
| Schema Mutation           | `0x1B88` | Unexpected data structure change    |
| Log Truncation            | `0x1C2B` | Critical log entries removed        |
| Packet Modification       | `0x2A90` | In-transit data tampering           |
| Timestamp Drift           | `0x2B11` | Significant clock skew              |
| API Spoofing              | `0x2C7F` | Impersonated service endpoint       |
| Prompt Injection          | `0x3A01` | Malicious input to LLM/system       |
| Entropy Leakage           | `0x3B99` | Cryptographic material exposed      |
| Register Forgery          | `0x3C4D` | Tampered hardware/software register |

## 🚀 Quick Start

```bash
git clone https://github.com/DeadLee702/adversarial-compliance-matrix.git
cd adversarial-compliance-matrix
cargo build --release
cargo run --bin gen_fixtures
cargo run --bin evk -- verify fixtures/incident_handoff_conflict.evkp