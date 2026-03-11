# ADR – Roadmap

Dieses Dokument beschreibt die Entwicklungsphasen von  
**ADR – Agent-Oriented Declarative Runtime**.

Die Roadmap ist bewusst inkrementell aufgebaut:

- zuerst Sicherheitskern
- dann überprüfbare Runtime
- danach erweiterte Policy- und Pipeline-Logik

Die Phasen sind keine starren Verträge, sondern Orientierungspunkte.

---

# Statusübersicht

## Bereits umgesetzt

- Phase 10 – Graph IR minimal
- Phase 11 – Audit Skeleton
- Phase 12 – Action Pipeline Test
- Phase 13 – Minimal Executor
- Phase 14 – Capability Enforcement
- Phase 15 – Audit Hash Chain

## Nächster geplanter Schritt

- Phase 16 – Policy Matching

---

# Phasenübersicht

## Phase 1–6: Architektur- und Spezifikationsphase

In den frühen Phasen wurden die grundlegenden Architekturideen entwickelt:

- Trennung zwischen Resolver und Runtime
- Safety-First-Prinzip
- KillSwitch / RuntimeState
- Auditierbarkeit
- Capability-Modell
- Open-Source-Ansatz

Diese Phasen existieren primär in `docs/dialog/`
und in den späteren ADR-Dokumenten.

---

## Phase 7 – Layer 2 Skeleton

Ziel:

- erster Resolver-Kern
- Policy-Skeleton
- Typdefinitionen
- grundlegende Compile-Fähigkeit

Ergebnis:

- `adr-layer2`
- Resolver-Trait
- Policy-Skeleton
- erste Tests

Status: **abgeschlossen**

---

## Phase 8 – Layer 1 Skeleton

Ziel:

- Runtime-Skeleton
- `RuntimeState`
- `CapabilitySet`
- `KillSwitchChannel`

Ergebnis:

- `adr-core`
- Runtime-Kern
- erste Integration mit Layer 2

Status: **abgeschlossen**

---

## Phase 9 – End-to-End Durchstich

Ziel:

- Resolver → Runtime Verbindung beweisen

Ergebnis:

- Resolver wählt minimalen Plan
- Runtime führt `execute_noop()` aus
- E2E Smoke Test

Status: **abgeschlossen**

---

## Phase 10 – Minimal Graph IR

Ziel:

- echtes Graph-Modell für Runtime und Resolver

Ergebnis:

- `NodeId`
- `Node`
- `ExecClass`
- `Effect`
- `Graph`
- `GraphHeader`

Zusätzlich:

- JSON Roundtrip Test

Status: **abgeschlossen**

---

## Phase 11 – Audit Skeleton

Ziel:

- erste Audit-Struktur

Ergebnis:

- `ActionKind`
- `Evidence`
- `ActionLogEntry`

Zusätzlich:

- Audit JSON Roundtrip Test

Status: **abgeschlossen**

---

## Phase 12 – Mini Action Pipeline

Ziel:

- Graph, Runtime und Audit in einer kleinen Pipeline verbinden

Ergebnis:

- Action Pipeline Test
- Runtime → Audit Grundpfad

Status: **abgeschlossen**

---

## Phase 13 – Minimal Executor

Ziel:

- erste echte Node-Ausführung

Ergebnis:

- `execute_node(&Node)`
- Unterscheidung zwischen:
  - `RealtimeSafe`
  - `Orchestrated`

Zusätzlich:

- Safety Tests für Realtime-Regeln
- Freeze-Blockierung vor Ausführung

Status: **abgeschlossen**

---

## Phase 14 – Capability Enforcement

Ziel:

- Capability-Prüfung im Executor erzwingen

Ergebnis:

- `Node` deklariert Capabilities
- Runtime besitzt aktives `CapabilitySet`
- `execute_node()` prüft Capabilities vor Ausführung

Zusätzlich:

- Test: fehlende Capability → `CapabilityNotGranted`
- Test: gewährte Capability → Ausführung erlaubt

Status: **abgeschlossen**

---

## Phase 15 – Audit Hash Chain

Ziel:

- Audit-Log manipulations-erkennbar machen

Ergebnis:

- `prev_hash`
- `entry_hash`
- SHA-256 basierte lineare Hash-Kette

Zusätzlich:

- Test: Hash-Kette verändert sich mit `prev_hash`

Status: **abgeschlossen**

---

## Phase 16 – Policy Matching

Ziel:

- Resolver stärker policy-aware machen

Mögliche Inhalte:

- Einschränkungen auf `Node.effect`
- Einschränkungen auf `Node.capabilities`
- Trust-Tier Anforderungen
- Domain-Policies

Offene Architekturfrage:

- Welche Teile der Policy müssen nur im Resolver liegen?
- Welche Teile sollten zusätzlich im Executor verifiziert werden?

Status: **geplant**

---

## Phase 17 – Multi-Node Execution Pipeline

Ziel:

- echte sequenzielle Ausführung von ExecutionPlans

Mögliche Inhalte:

- mehrere Nodes pro Plan
- deterministische Reihenfolge
- Checkpoints
- spätere Scheduler-Kontexte

Status: **geplant**

---

## Phase 18 – Human Gate / Checkpoint Flow

Ziel:

- kontrollierte menschliche Freigaben in der Pipeline

Mögliche Inhalte:

- Human Approval Gates
- Timeout-Handling
- persistente Zustandsübergänge

Hinweis:

Diese Phase ist bewusst spät angesetzt,
damit der Sicherheitskern vorher stabil ist.

Status: **geplant**

---

## Mögliche spätere Phasen

Spätere Ausbaustufen könnten enthalten:

- Merkle Trees für Audit-Batching
- externer Anchor / Timestamp Service
- persistente Log-Speicherung
- Scheduler-Kontext-Modell
- Multi-Party Authorization
- formale Verifikation einzelner Safety-Invarianten

Status: **offen**

---

# Entwicklungsprinzip

Die Roadmap folgt einem Grundsatz:

**Safety vor Komfort.**

Das bedeutet:

- zuerst Runtime-Enforcement
- dann Auditierbarkeit
- erst später Interaktions- und Komfortfunktionen

---

# Dokumentationsbezug

Diskussionen und Reviews:

```text
docs/dialog/

Verbindliche Architekturentscheidungen:

docs/adr/

docs/PROJECT_VISION.md
docs/ARCHITECTURE.md
docs/SAFETY_MODEL.md
docs/THREAT_MODEL.md
