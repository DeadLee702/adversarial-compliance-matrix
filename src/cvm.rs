// Adversarial Compliance Status Codes
pub const STATUS_HANDOFF_CONFLICT: u16 = 0x0F2E;
pub const STATUS_RACE_CONDITION: u16 = 0x0E1A;
pub const STATUS_ORPHANED_STEP: u16 = 0x0D44;
pub const STATUS_TRANSACTION_REPLAY: u16 = 0x1A4F;
pub const STATUS_SCHEMA_MUTATION: u16 = 0x1B88;
pub const STATUS_LOG_TRUNCATION: u16 = 0x1C2B;
pub const STATUS_PACKET_MODIFICATION: u16 = 0x2A90;
pub const STATUS_TIMESTAMP_DRIFT: u16 = 0x2B11;
pub const STATUS_API_SPOOFING: u16 = 0x2C7F;
pub const STATUS_PROMPT_INJECTION: u16 = 0x3A01;
pub const STATUS_ENTROPY_LEAKAGE: u16 = 0x3B99;
pub const STATUS_REGISTER_FORGERY: u16 = 0x3C4D;

/// Compliance verdict — the enforcement action the Kill Vector should take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Healthy — allow execution
    Pura,
    /// Warning — allow but flag for review
    Vigla,
    /// Compromised — block and enforce containment
    Poluita,
}

impl Verdict {
    pub fn as_str(&self) -> &'static str {
        match self {
            Verdict::Pura => "PURA",
            Verdict::Vigla => "VIGLA",
            Verdict::Poluita => "POLUITA",
        }
    }

    pub fn enforcement_action(&self) -> &'static str {
        match self {
            Verdict::Pura => "allow",
            Verdict::Vigla => "warn",
            Verdict::Poluita => "block",
        }
    }
}

/// Verification result structure
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub status_code: u16,
    pub is_valid: bool,
    pub message: String,
}

impl VerificationResult {
    pub fn new(status_code: u16, is_valid: bool, message: impl Into<String>) -> Self {
        VerificationResult {
            status_code,
            is_valid,
            message: message.into(),
        }
    }

    pub fn format_output(&self) -> String {
        let status_str = format!("0x{:04X}", self.status_code);
        let validity = if self.is_valid { "VALID" } else { "INVALID" };
        format!("{}: {} [{}]", validity, self.message, status_str)
    }
}

/// Compliance report — the full output of the compliance engine
#[derive(Debug, Clone)]
pub struct ComplianceReport {
    pub verdict: Verdict,
    pub status_code: u16,
    pub incident_type: String,
    pub message: String,
    pub recommended_action: String,
    pub severity: String,
    pub confidence: f32,
    pub enforcement_action: String,
    pub timestamp: String,
}

impl ComplianceReport {
    pub fn to_json(&self) -> String {
        serde_json::json!({
            "verdict": self.verdict.as_str(),
            "enforcement_action": self.enforcement_action,
            "status_code": format!("0x{:04X}", self.status_code),
            "incident_type": self.incident_type,
            "message": self.message,
            "recommended_action": self.recommended_action,
            "severity": self.severity,
            "confidence": self.confidence,
            "timestamp": self.timestamp,
        })
        .to_string()
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&serde_json::json!({
            "verdict": self.verdict.as_str(),
            "enforcement_action": self.enforcement_action,
            "status_code": format!("0x{:04X}", self.status_code),
            "incident_type": self.incident_type,
            "message": self.message,
            "recommended_action": self.recommended_action,
            "severity": self.severity,
            "confidence": self.confidence,
            "timestamp": self.timestamp,
        }))
        .unwrap_or_else(|_| "{}".to_string())
    }
}

/// The compliance engine — evaluates a status code and generates a verdict.
///
/// Verdict mapping:
///   0x0000 (clean)                 -> PURA  (allow)
///   MEDIUM severity incidents      -> VIGLA (warn)
///   HIGH/CRITICAL severity         -> POLUITA (block)
pub fn evaluate_compliance(status_code: u16) -> ComplianceReport {
    let timestamp = chrono::Utc::now().to_rfc3339();

    if status_code == 0x0000 {
        return ComplianceReport {
            verdict: Verdict::Pura,
            status_code,
            incident_type: "Clean".to_string(),
            message: "Clean artifact".to_string(),
            recommended_action: "allow".to_string(),
            severity: "LOW".to_string(),
            confidence: 1.0,
            enforcement_action: "allow".to_string(),
            timestamp,
        };
    }

    let (incident_type, message, severity, recommended_action) = match status_code {
        STATUS_HANDOFF_CONFLICT => ("Handoff Conflict", "Handoff conflict detected", "HIGH", "block"),
        STATUS_RACE_CONDITION => ("Race Condition", "Race condition detected", "HIGH", "quarantine"),
        STATUS_ORPHANED_STEP => ("Orphaned Step", "Orphaned step detected", "MEDIUM", "escalate"),
        STATUS_TRANSACTION_REPLAY => ("Transaction Replay", "Transaction replay detected", "CRITICAL", "block"),
        STATUS_SCHEMA_MUTATION => ("Schema Mutation", "Schema mutation detected", "MEDIUM", "quarantine"),
        STATUS_LOG_TRUNCATION => ("Log Truncation", "Log truncation detected", "CRITICAL", "escalate"),
        STATUS_PACKET_MODIFICATION => ("Packet Modification", "Packet modification detected", "HIGH", "block"),
        STATUS_TIMESTAMP_DRIFT => ("Timestamp Drift", "Timestamp drift detected", "MEDIUM", "escalate"),
        STATUS_API_SPOOFING => ("API Spoofing", "API spoofing detected", "CRITICAL", "block"),
        STATUS_PROMPT_INJECTION => ("Prompt Injection", "Prompt injection detected", "HIGH", "quarantine"),
        STATUS_ENTROPY_LEAKAGE => ("Entropy Leakage", "Entropy leakage detected", "CRITICAL", "escalate"),
        STATUS_REGISTER_FORGERY => ("Register Forgery", "Register forgery detected", "CRITICAL", "block"),
        _ => ("Unknown Incident", "Unknown malicious status detected", "MEDIUM", "escalate"),
    };

    let verdict = match severity {
        "CRITICAL" | "HIGH" => Verdict::Poluita,
        "MEDIUM" => Verdict::Vigla,
        _ => Verdict::Pura,
    };

    let enforcement_action = verdict.enforcement_action();

    ComplianceReport {
        verdict,
        status_code,
        incident_type: incident_type.to_string(),
        message: message.to_string(),
        recommended_action: recommended_action.to_string(),
        severity: severity.to_string(),
        confidence: 0.90,
        enforcement_action: enforcement_action.to_string(),
        timestamp,
    }
}

/// Verify a file and produce a compliance report.
pub fn verify_file(path: &str) -> Result<ComplianceReport, String> {
    let file_data =
        std::fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;

    if file_data.len() < 2 {
        let mut report = evaluate_compliance(0x0000);
        report.verdict = Verdict::Vigla;
        report.message = "File too short to contain status code".to_string();
        report.severity = "MEDIUM".to_string();
        report.enforcement_action = "warn".to_string();
        report.incident_type = "Malformed".to_string();
        return Ok(report);
    }

    let status_code = u16::from_be_bytes([file_data[0], file_data[1]]);
    Ok(evaluate_compliance(status_code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_codes_are_unique() {
        let codes = vec![
            STATUS_HANDOFF_CONFLICT, STATUS_RACE_CONDITION, STATUS_ORPHANED_STEP,
            STATUS_TRANSACTION_REPLAY, STATUS_SCHEMA_MUTATION, STATUS_LOG_TRUNCATION,
            STATUS_PACKET_MODIFICATION, STATUS_TIMESTAMP_DRIFT, STATUS_API_SPOOFING,
            STATUS_PROMPT_INJECTION, STATUS_ENTROPY_LEAKAGE, STATUS_REGISTER_FORGERY,
        ];
        let mut sorted = codes.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(codes.len(), sorted.len(), "Status codes must be unique");
    }

    #[test]
    fn test_clean_code_produces_pura() {
        let report = evaluate_compliance(0x0000);
        assert_eq!(report.verdict, Verdict::Pura);
        assert_eq!(report.enforcement_action, "allow");
    }

    #[test]
    fn test_critical_incident_produces_poluita() {
        let report = evaluate_compliance(STATUS_TRANSACTION_REPLAY);
        assert_eq!(report.verdict, Verdict::Poluita);
        assert_eq!(report.enforcement_action, "block");
    }

    #[test]
    fn test_medium_incident_produces_vigla() {
        let report = evaluate_compliance(STATUS_ORPHANED_STEP);
        assert_eq!(report.verdict, Verdict::Vigla);
        assert_eq!(report.enforcement_action, "warn");
    }

    #[test]
    fn test_high_incident_produces_poluita() {
        let report = evaluate_compliance(STATUS_HANDOFF_CONFLICT);
        assert_eq!(report.verdict, Verdict::Poluita);
        assert_eq!(report.enforcement_action, "block");
    }

    #[test]
    fn test_unknown_code_produces_vigla() {
        let report = evaluate_compliance(0xFFFF);
        assert_eq!(report.verdict, Verdict::Vigla);
        assert_eq!(report.enforcement_action, "warn");
    }

    #[test]
    fn test_report_json_output() {
        let report = evaluate_compliance(STATUS_PROMPT_INJECTION);
        let json = report.to_json();
        assert!(json.contains("POLUITA"));
        assert!(json.contains("block"));
        assert!(json.contains("Prompt Injection"));
    }
}
