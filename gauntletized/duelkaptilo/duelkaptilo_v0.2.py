#!/usr/bin/env python3
"""
Protocol 10: Duelkaptilo
DEC FORCE v2.2 Dual Verification

Research/Educational - SIMULADO mode only - No destructive operations - Dual verification only

Counts files and total size using two independent methods (os.walk vs pathlib.glob).
Flags discrepancies between methods to detect listing anomalies.
"""

import sys
import json
import argparse
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Any, Optional, Tuple
from enum import Enum
import uuid
import os


class Verdict(Enum):
    PURA = "PURA"
    ALARMO = "ALARMO"
    PERJURO_DETEKTITA = "PERJURO_DETEKTITA"


class EsperantoProtokolo:
    def __init__(self, protocol_name: str, run_id: Optional[str] = None):
        self.protocol_name = protocol_name
        self.run_id = run_id or str(uuid.uuid4())[:8]
        self.timestamp = datetime.utcnow().isoformat() + "Z"
        self.logs: List[Dict[str, Any]] = []
        self.verdict = Verdict.PURA

    def log(self, event: str, data: Optional[Dict[str, Any]] = None) -> None:
        entry = {
            "timestamp": datetime.utcnow().isoformat() + "Z",
            "event": event,
            "run_id": self.run_id,
            "data": data or {}
        }
        self.logs.append(entry)

    def set_verdict(self, verdict: Verdict) -> None:
        self.verdict = verdict
        self.log("VERDICT_SET", {"verdict": verdict.value})


class Duelkaptilo:
    def __init__(
        self,
        target_path: str,
        output_file: Optional[str] = None,
        simulado: bool = True
    ):
        self.target_path = Path(target_path)
        self.output_file = Path(output_file) if output_file else None
        self.simulado = simulado
        self.logger = EsperantoProtokolo("Duelkaptilo")
        self.discrepancies: List[Dict[str, Any]] = []

    def validate_simulado(self) -> bool:
        if not self.simulado:
            self.logger.set_verdict(Verdict.PERJURO_DETEKTITA)
            self.logger.log("SIMULADO_DISABLED", {"error": "SIMULADO mode required"})
            return False
        return True

    def validate_target(self) -> bool:
        if not self.target_path.exists():
            self.logger.log("TARGET_NOT_FOUND", {"target": str(self.target_path)})
            self.logger.set_verdict(Verdict.ALARMO)
            return False
        return True

    def count_via_os_walk(self, directory: Path) -> Tuple[int, int]:
        """Count files and total size using os.walk"""
        file_count = 0
        total_size = 0
        try:
            for root, dirs, files in os.walk(str(directory)):
                for file in files:
                    file_path = os.path.join(root, file)
                    try:
                        if not os.path.islink(file_path):
                            total_size += os.path.getsize(file_path)
                            file_count += 1
                    except Exception as e:
                        self.logger.log("OSWALK_FILE_ERROR", {"file": file_path})
        except Exception as e:
            self.logger.log("OSWALK_ERROR", {"error": str(e)})
        return file_count, total_size

    def count_via_pathlib(self, directory: Path) -> Tuple[int, int]:
        """Count files and total size using pathlib.glob"""
        file_count = 0
        total_size = 0
        try:
            for file_path in directory.rglob("*"):
                if file_path.is_file() and not file_path.is_symlink():
                    try:
                        total_size += file_path.stat().st_size
                        file_count += 1
                    except Exception as e:
                        self.logger.log("PATHLIB_FILE_ERROR", {"file": str(file_path)})
        except Exception as e:
            self.logger.log("PATHLIB_ERROR", {"error": str(e)})
        return file_count, total_size

    def verify(self) -> int:
        try:
            if not self.validate_simulado():
                return 2
            if not self.validate_target():
                return 1

            self.logger.log("DUELKAPTILO_START", {
                "target": str(self.target_path),
                "mode": "SIMULADO"
            })

            if self.target_path.is_file():
                # For single file, both methods should agree
                try:
                    size = self.target_path.stat().st_size
                    oswalk_count = 1
                    oswalk_size = size
                    pathlib_count = 1
                    pathlib_size = size
                except Exception as e:
                    self.logger.log("FILE_STAT_ERROR", {"file": str(self.target_path)})
                    return 1
            else:
                oswalk_count, oswalk_size = self.count_via_os_walk(self.target_path)
                pathlib_count, pathlib_size = self.count_via_pathlib(self.target_path)

            # Check for discrepancies
            if oswalk_count != pathlib_count:
                self.discrepancies.append({
                    "type": "file_count_mismatch",
                    "os_walk": oswalk_count,
                    "pathlib": pathlib_count,
                    "difference": abs(oswalk_count - pathlib_count)
                })

            if oswalk_size != pathlib_size:
                self.discrepancies.append({
                    "type": "total_size_mismatch",
                    "os_walk": oswalk_size,
                    "pathlib": pathlib_size,
                    "difference": abs(oswalk_size - pathlib_size)
                })

            # Determine verdict
            if self.discrepancies:
                verdict = Verdict.ALARMO
                self.logger.log("DUAL_VERIFICATION_DISCREPANCY", {
                    "count": len(self.discrepancies),
                    "discrepancies": self.discrepancies
                })
            else:
                verdict = Verdict.PURA
                self.logger.log("DUAL_VERIFICATION_MATCH", {
                    "file_count": oswalk_count,
                    "total_size": oswalk_size
                })

            self.logger.set_verdict(verdict)

            report = {
                "protocol": "duelkaptilo_v0.2",
                "protocol_number": 10,
                "timestamp": self.logger.timestamp,
                "target": str(self.target_path),
                "mode": "SIMULADO",
                "os_walk_count": oswalk_count,
                "os_walk_size": oswalk_size,
                "pathlib_count": pathlib_count,
                "pathlib_size": pathlib_size,
                "discrepancies": self.discrepancies,
                "discrepancy_count": len(self.discrepancies),
                "verdict": verdict.value,
                "warning": "Neniu dosiero estis modifita",
                "orchestrator_logs": self.logger.logs,
                "closure": "Relenthol engaĝita."
            }

            report_json = json.dumps(report, indent=2)
            if self.output_file:
                self.output_file.write_text(report_json)
            else:
                print(report_json)

            self.logger.log("DUELKAPTILO_END", {
                "verdict": verdict.value,
                "discrepancy_count": len(self.discrepancies)
            })

            return 0 if verdict == Verdict.PURA else 1

        except Exception as e:
            self.logger.log("DUELKAPTILO_ERROR", {"error": str(e)})
            self.logger.set_verdict(Verdict.ALARMO)

            error_report = {
                "protocol": "duelkaptilo_v0.2",
                "protocol_number": 10,
                "timestamp": datetime.utcnow().isoformat() + "Z",
                "target": str(self.target_path),
                "verdict": Verdict.ALARMO.value,
                "error": str(e),
                "orchestrator_logs": self.logger.logs,
                "closure": "Relenthol engaĝita."
            }

            error_json = json.dumps(error_report, indent=2)
            if self.output_file:
                self.output_file.write_text(error_json)
            else:
                print(error_json)

            return 1


def main():
    parser = argparse.ArgumentParser(
        description="Protocol 10: Duelkaptilo - DEC FORCE v2.2 Dual Verification"
    )

    parser.add_argument(
        "--simulado",
        type=lambda x: x.lower() in ("true", "1", "yes"),
        default=True,
        help="Run in SIMULADO (read-only simulator) mode [default: True]"
    )

    parser.add_argument(
        "--target",
        type=str,
        required=True,
        help="Directory or file to verify (required)"
    )

    parser.add_argument(
        "--output",
        type=str,
        default=None,
        help="Output JSON file [default: stdout]"
    )

    args = parser.parse_args()

    verifier = Duelkaptilo(
        target_path=args.target,
        output_file=args.output,
        simulado=args.simulado
    )

    exit_code = verifier.verify()
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
