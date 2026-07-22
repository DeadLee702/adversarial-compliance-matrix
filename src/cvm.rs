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

    /// Format output for verification with hex code display
    pub fn format_output(&self) -> String {
        let status_str = format!("0x{:04X}", self.status_code);
        let validity = if self.is_valid { "VALID" } else { "INVALID" };
        format!("{}: {} [{}]", validity, self.message, status_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_codes_are_unique() {
        let codes = vec![
            STATUS_HANDOFF_CONFLICT,
            STATUS_RACE_CONDITION,
            STATUS_ORPHANED_STEP,
            STATUS_TRANSACTION_REPLAY,
            STATUS_SCHEMA_MUTATION,
            STATUS_LOG_TRUNCATION,
            STATUS_PACKET_MODIFICATION,
            STATUS_TIMESTAMP_DRIFT,
            STATUS_API_SPOOFING,
            STATUS_PROMPT_INJECTION,
            STATUS_ENTROPY_LEAKAGE,
            STATUS_REGISTER_FORGERY,
        ];

        let mut sorted = codes.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(codes.len(), sorted.len(), "Status codes must be unique");
    }
}
