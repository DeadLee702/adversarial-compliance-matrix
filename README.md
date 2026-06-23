# adversarial-compliance-matrix - EVK Stack Test Harness

Adversarial testing framework for EVK Stack evidence bundles. Validates cryptographic signing, verification, and fail-closed behavior.

## Core Function
1. **Load**: Parse `.evkp` bundles from evk
2. **Verify**: Call gemini-box to verify ed25519 signatures  
3. **Test**: Run adversarial cases - tamper, missing sig, invalid keys
4. **Report**: Output PASS/FAIL matrix for compliance validation

## Usage
```bash
cargo test --test compliance --nocapture
```

## EVK Stack Integration
`evk` → bundles → `gemini-box` → signs → `adversarial-compliance-matrix` → tests

MIT Licensed. Part of EVK Stack.