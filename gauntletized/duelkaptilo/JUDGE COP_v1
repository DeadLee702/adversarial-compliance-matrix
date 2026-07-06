#!/usr/bin/env python3
"""DEC FORCE 10 - JUDGE COP_v1"""
import json, sys

GAUNTLET_REPORT = sys.argv[1] if len(sys.argv) > 1 else "MHA_FINAL_REPORT.json"

with open(GAUNTLET_REPORT) as f:
    report = json.load(f)

# Rule 1: EVK Core must be VERIFIED
if report["audit_results"]["evk_core"]!= "VERIFIED":
    print("[JUDGE] HALT: EVK Core malpura. Kitchen is dirty.")
    sys.exit(2)

# Rule 2: COP Score threshold from health_score if present
cop_score = report.get("health_score", 0.0)
if cop_score > 15.0:
    print(f"[JUDGE] HALT: COP Score {cop_score}% exceeds 15% threshold.")
    print(f"[JUDGE] VERDICT: MALPURA")
    sys.exit(1)

# Rule 3: All must be VERIFIED
if report["status"]!= "GAUNTLET_COMPLETE":
    print("[JUDGE] HALT: Gauntlet incomplete.")
    sys.exit(1)

print(f"[JUDGE] COP: {cop_score}% | VERDICT: PURA")
print(f"[JUDGE] Lingvo sen esceptoj. Relenthol engaĝita.")
sys.exit(0)
