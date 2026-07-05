# DEC FORCE v2.2 (Development Phase - Research/Educational)

**Status:** Active Research Project | **Hackathon:** FIND EVIL! (July 2026)

---

## Purpose

DEC FORCE v2.2 is a **research-phase forensic simulation framework** built for educational demonstration and hackathon participation. It is **not intended for production environments** and should only be used in controlled, isolated test settings.

This framework explores distributed audit architectures using read-only forensic analysis patterns. All protocols operate in **SIMULADO (Simulator) mode** with no destructive capabilities.

---

## Architecture Overview

DEC FORCE v2.2 is **distributed across three GitHub repositories** in a federated architecture:

### Repository Distribution

| Repository | Role | Protocols |
|---|---|---|
| **evk** | Core forensic orchestration | 1, 2, 3, 5, 6 |
| **gemini-box** | Text analysis + logging engine | 4, 8 |
| **adversarial-compliance-matrix** | Compliance monitoring + simulators | 7, 9, 10 |

### The 10 Protocols

```
gauntletized/
├── oracle/               (Protocol 1: Pendulastika - Entropy analysis)
├── alighostest/          (Protocol 2: Fantomlumo - Hidden file detection)
├── bridge/               (Protocol 3: Brajloskripto - System snapshots)
├── perjanocyst/          (Protocol 4: Perjurocisto - Text analysis) [Gemini-Box]
├── trapzonar/            (Protocol 5: Kaptilradaro - Tripwire monitoring)
├── kitchzensync/         (Protocol 6: Kuirejsinkronigo - Orchestrator)
├── detruquinzarian/      (Protocol 7: Detruanto - FS simulator) [Compliance]
├── esperanto/            (Protocol 8: Esperanto Engine - Core logging) [Gemini-Box]
├── tempokapsulo/         (Protocol 9: Tempokapsulo - Time snapshots) [Compliance]
└── duelkaptilo/          (Protocol 10: Duelkaptilo - Dual monitoring) [Compliance]
```

### Central Link: Esperanto Engine

All protocols depend on **Protocol 8 (Esperanto Engine)**, deployed in `gemini-box/gauntletized/esperanto/esperanto_engine.py`.

This module provides:
- Unified logging (EsperantoProtokolo class)
- Verdict system (PURA, ALARMO, PERJURO_DETEKTITA)
- JSON report generation
- Mandatory linguistic closure: "Relenthol engaĝita."

---

## Execution Guide

### Prerequisites

- Python 3.8+
- Git (for cloning repositories)
- All three repositories cloned locally

### Running Individual Protocols

Each protocol can be executed independently from its folder:

#### EVK Repository (Protocols 1, 2, 3, 5, 6)

```bash
cd evk/gauntletized

# Protocol 1: Pendulastika Oracle
python oracle/oracle_v0.2.py

# Protocol 2: Fantomlumo
python alighostest/alighostest_v0.2.py

# Protocol 3: Brajloskripto
python bridge/bridge_v0.2.py

# Protocol 5: Kaptilradaro
python trapzonar/trapzonar_v0.2.py

# Protocol 6: Kuirejsinkronigo (Orchestrator)
python kitchzensync/kitchzensync_v0.2.py
```

#### Gemini-Box Repository (Protocols 4, 8)

```bash
cd gemini-box/gauntletized

# Protocol 8: Esperanto Engine (Logging foundation - import only)
python -c "from esperanto.esperanto_engine import EsperantoProtokolo; print('Esperanto Engine loaded')"

# Protocol 4: Perjurocisto
python perjanocyst/perjurocisto_v0.2.py
```

#### Adversarial-Compliance-Matrix Repository (Protocols 7, 9, 10)

```bash
cd adversarial-compliance-matrix/gauntletized

# Protocol 7: Detruanto
python detruquinzarian/detruquinzarian_v0.2.py

# Protocol 9: Tempokapsulo
python tempokapsulo/tempokapsulo_v0.2.py

# Protocol 10: Duelkaptilo
python duelkaptilo/duelkaptilo_v0.2.py
```

---

## Educational Health Check: mha_run.sh

The **Master Health Assessment (MHA)** is an educational verification script designed for development environments only. It checks that all protocol folders exist and the Esperanto engine is accessible.

```bash
#!/bin/bash
# DEC FORCE v2.2 - Master Health Assessment (Educational Use Only)
# Verifies distributed architecture state across all three repositories

set -e

echo "=== DEC FORCE v2.2 Master Health Assessment ==="
echo "Status: DEVELOPMENT PHASE"
echo ""

# Verify EVK
echo "[EVK] Checking Protocol Folders..."
for proto in oracle alighostest bridge trapzonar kitchzensync; do
  if [ -d "evk/gauntletized/$proto" ]; then
    echo "  ✓ $proto folder exists"
  else
    echo "  ✗ $proto folder missing"
  fi
done

# Verify Gemini-Box
echo ""
echo "[Gemini-Box] Checking Esperanto Engine..."
if [ -f "gemini-box/gauntletized/esperanto/esperanto_engine.py" ]; then
  echo "  ✓ Esperanto Engine deployed"
else
  echo "  ✗ Esperanto Engine missing"
fi

# Verify Compliance Matrix
echo ""
echo "[Compliance] Checking Protocol Folders..."
for proto in detruquinzarian tempokapsulo duelkaptilo; do
  if [ -d "adversarial-compliance-matrix/gauntletized/$proto" ]; then
    echo "  ✓ $proto folder exists"
  else
    echo "  ✗ $proto folder missing"
  fi
done

echo ""
echo "=== Health Assessment Complete ==="
echo "Relenthol engaĝita."
```

### Running MHA

```bash
chmod +x mha_run.sh
./mha_run.sh
```

**Output:** Confirms that the distributed architecture is in place and ready for protocol development.

---

## Current Implementation Status

### ✅ Deployed

- **Specification Document:** `.dec-force-spec.md` in all three repositories
- **Folder Structure:** All 10 protocol directories created
- **Protocol 8 (Esperanto Engine):** Fully functional logging module
  - `EsperantoProtokolo` class: verdict system, JSON reporting
  - `ProtocolOrchestrator` base class: multi-protocol coordination
  - Ready for import by all other protocols

### 🔄 In Development

- **Protocol 1 (Pendulastika):** Entropy analysis framework (pending)
- **Protocol 2 (Fantomlumo):** Hidden file detection logic (pending)
- **Protocol 3 (Brajloskripto):** System snapshot collection (pending)
- **Protocol 4 (Perjurocisto):** Text scanning implementation (pending)
- **Protocol 5 (Kaptilradaro):** Tripwire monitoring setup (pending)
- **Protocol 6 (Kitchzensync):** Multi-protocol orchestration (pending)
- **Protocol 7 (Detruanto):** Filesystem simulator (pending)
- **Protocol 9 (Tempokapsulo):** Time-based snapshots (pending)
- **Protocol 10 (Duelkaptilo):** Dual-layer comparison (pending)

---

## Safety & Compliance

### SIMULADO Mode (Simulator-Only)

All DEC FORCE v2.2 protocols operate in **SIMULADO mode**—a read-only simulation environment.

#### ✅ Allowed Operations

- File reading and analysis
- JSON report generation
- Logging and telemetry
- System introspection (read-only queries)
- Pattern matching and detection simulation

#### ❌ Forbidden Operations

- `os.remove()` — No file deletion
- `shutil.rmtree()` — No directory deletion
- `ctypes` — No kernel access
- Direct hardware memory manipulation
- Any destructive system modifications

### Non-Destructive Guarantee

Every protocol report includes:
```json
{
  "warning": "Neniu dosiero estis modifita",
  "message": "No files were modified"
}
```

This warning appears in all `.json` report files generated by each protocol.

### Linguistic Closure

All protocol outputs end with the mandatory closure:
```
Relenthol engaĝita.
```

This Esperanto phrase confirms protocol completion and adherence to safety constraints.

---

## Specification Reference

The authoritative architecture specification is defined in:

```
.dec-force-spec.md
```

Located in all three repositories (`evk`, `gemini-box`, `adversarial-compliance-matrix`).

Key sections:
- **The Architecture:** Strict folder mapping
- **Linguistic Rules:** Verdict system and logging protocol
- **Safety Mandate:** Hard constraints and output requirements
- **Protocol Specifications:** Individual protocol purposes
- **Implementation Rules:** Folder structure and import chains

---

## Roadmap

| Phase | Status | Description |
|---|---|---|
| **Phase 1** | ✅ Complete | Specification + Folder Structure |
| **Phase 2** | ✅ Complete | Esperanto Engine deployment |
| **Phase 3** | 🔄 In Progress | Individual protocol implementations |
| **Phase 4** | Pending | Integration testing |
| **Phase 5** | Pending | Hackathon demonstration |

---

## Development Notes

- This is **research-grade code** intended for educational exploration
- All protocols are **read-only forensic simulations**
- The framework is **distributed by design** for federated audit scenarios
- **No production use** is recommended or supported
- Use only in **isolated development environments**

---

## License & Attribution

DEC FORCE v2.2 is developed as part of the **FIND EVIL! Hackathon (July 2026)**.

**Architect:** DeadLee702

**Repositories:**
- https://github.com/DeadLee702/evk
- https://github.com/DeadLee702/gemini-box
- https://github.com/DeadLee702/adversarial-compliance-matrix

---

**Lingvo sen esceptoj. Relenthol engaĝita.**
