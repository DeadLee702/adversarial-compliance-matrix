# Adversarial Compliance Matrix (ACM)

> Continuous runtime validation and compliance engine for the Z-12 Sovereign Security Platform.

## Role in the Z-12 Platform

| Component | Role |
|-----------|------|
| **EVK** | Deterministic identity/integrity verification + Kill Vector enforcement |
| **Gemini-Box** | ed25519 signing, non-repudiation, forensic analysis |
| **ACM** (this repo) | Compliance engine, verdict generation, threat detection |

## Architecture

```
Gemini-Box output → ACM evaluates → Verdict (PURA/VIGLA/POLUITA) → Kill Vector enforces
```

## Compliance Engine

```rust
use adversarial_compliance_matrix::cvm::{evaluate_compliance, Verdict};

let report = evaluate_compliance(0x0F2E);
assert_eq!(report.verdict, Verdict::Poluita);
assert_eq!(report.enforcement_action, "block");
```

### Verdict Mapping

| Verdict | Severity | Enforcement Action | Kill Vector |
|---------|----------|-------------------|-------------|
| **PURA** | LOW | Allow | No action |
| **VIGLA** | MEDIUM | Warn | Flag for review |
| **POLUITA** | HIGH/CRITICAL | Block | SIGKILL + forensic log |

## CLI Usage

```bash
cargo run --release --bin gen_fixtures
./target/release/evk verify test/incident_clean.evkp
./target/release/evk verify test/incident_7f3a.evkp --json
cargo test --release  # 28 tests
```

## Status Code Reference

| Code | Incident | Severity | Verdict | Action |
|------|----------|----------|---------|--------|
| 0x0000 | Clean | LOW | PURA | allow |
| 0x0F2E | Handoff Conflict | HIGH | POLUITA | block |
| 0x0E1A | Race Condition | HIGH | POLUITA | quarantine |
| 0x0D44 | Orphaned Step | MEDIUM | VIGLA | escalate |
| 0x1A4F | Transaction Replay | CRITICAL | POLUITA | block |
| 0x1B88 | Schema Mutation | MEDIUM | VIGLA | quarantine |
| 0x1C2B | Log Truncation | CRITICAL | POLUITA | escalate |
| 0x2A90 | Packet Modification | HIGH | POLUITA | block |
| 0x2B11 | Timestamp Drift | MEDIUM | VIGLA | escalate |
| 0x2C7F | API Spoofing | CRITICAL | POLUITA | block |
| 0x3A01 | Prompt Injection | HIGH | POLUITA | quarantine |
| 0x3B99 | Entropy Leakage | CRITICAL | POLUITA | escalate |
| 0x3C4D | Register Forgery | CRITICAL | POLUITA | block |

## JSON Output Format

```json
{
  "verdict": "POLUITA",
  "enforcement_action": "block",
  "status_code": "0x0F2E",
  "incident_type": "Handoff Conflict",
  "message": "Handoff conflict detected",
  "recommended_action": "block",
  "severity": "HIGH",
  "confidence": 0.9,
  "timestamp": "2026-07-23T10:03:16.537389668+00:00"
}
```

MIT Licensed. Part of the Z-12 platform.
