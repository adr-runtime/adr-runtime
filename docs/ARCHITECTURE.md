# ADR – Agent-Oriented Declarative Runtime
## Architecture Overview

ADR ist eine sichere Runtime für KI-Agenten.  
Das System trennt Planung (Resolver) von sicherheitskritischer Ausführung (Runtime).

Diese Trennung stellt sicher, dass selbst fehlerhafte Resolver keine
unsicheren Operationen ausführen können.

---

# High-Level Architecture
Intent / Request
│
▼
Resolver (Layer 2)
│
▼
ExecutionPlan
│
▼
Runtime / Executor (Layer 1)
│
▼
Capability Enforcement
│
▼
ExecClass + Effect Validation
│
▼
Audit Log (Hash Chain)


Layer 2 erzeugt Vorschläge.  
Layer 1 entscheidet, ob diese Vorschläge sicher sind.

---

# Layer Model

## Layer 2 – Resolver

Der Resolver interpretiert Intents und erstellt einen Plan.

Typische Komponenten:

- IntentNode
- Resolver
- Policy Matching
- Graph Planning

Resolver-Code darf **keine direkten Aktionen ausführen**.

---

## Layer 1 – Runtime Kernel

Der Runtime-Kern führt Nodes deterministisch aus und erzwingt Sicherheitsregeln.

Komponenten:

- RuntimeState
- CapabilitySet
- Executor
- KillSwitch
- Audit Log

Dieser Layer ist der **Safety Kernel**.

---

# Runtime Safety Pipeline

Jede Node-Ausführung durchläuft diese Reihenfolge:

poll_kill_switch()
│
RuntimeState Check
│
Capability Enforcement
│
ExecClass Validation
│
Effect Validation
│
Execution
│
Audit Logging


Diese Reihenfolge stellt sicher, dass keine unsichere Operation ausgeführt wird.

---

# Runtime States

Die Runtime besitzt eine Sicherheitsordnung:

Running < Stopping < Halted < Frozen


Je höher der Zustand, desto stärker die Einschränkung.

Beispiele:

- Running → normale Operation
- Halted → keine Node-Ausführung
- Frozen → vollständiger Sicherheitsstopp

---

# Capability Model

Nodes deklarieren Capabilities, die sie benötigen.

Beispiele:

- filesystem_write
- network_external
- actuator_control

Der Executor prüft vor jeder Node-Ausführung:

node.capabilities ⊆&sube; runtime.capability_set

Fehlende Capabilities führen zu:

CapabilityNotGranted


---

# Execution Classes

Nodes besitzen eine Execution Class:

RealtimeSafe
Orchestrated


RealtimeSafe Nodes dürfen **keine externen Effekte** haben.

Beispiel:


RealtimeSafe + Effect::None → erlaubt
RealtimeSafe + Effect::FsWrite → Fehler


---

# Audit Logging

Alle Aktionen werden in einem Audit Log gespeichert.

Jeder Eintrag enthält:

- node_id
- action_kind
- timestamp
- evidence
- prev_hash
- entry_hash

Die Einträge bilden eine lineare Hash-Kette.

entry1
entry2 → hash(entry1)
entry3 → hash(entry2)


Dadurch wird nachträgliche Manipulation erkennbar.

---

# Safety Philosophy

ADR ist **kein Ersatz für deterministische Low-Level-Systeme**.

ADR entscheidet:

Darf ein Agent diese Operation ausführen?

Nicht:

Wie wird die Operation technisch ausgeführt?


Beispiel:

ADR erlaubt:

actuator_control


Die tatsächliche Hardwaresteuerung erfolgt in einem
separaten deterministischen System.

---

# Repository Structure

docs/
├─ adr/ Architecture Decision Records
├─ dialog/ ChatGPT ↔ Claude Architektur-Dialog
└─ ARCHITECTURE.md

crates/
├─ adr-core
└─ adr-layer2


---

# Development Model

Die Architektur entsteht durch:

- Implementierung
- Architektur-Dialog
- Architecture Decision Records (ADR)

Neue Entscheidungen werden dokumentiert in:

docs/adr/


Diskussionen und Reviews befinden sich in:

docs/dialog/


---

# Future Roadmap

Nächste geplante Phasen:

Phase 16 – Policy Matching  
Phase 17 – Multi-Node Execution Pipeline  
Phase 18 – Human Gate / Checkpoint System  
Phase 19 – Merkle Audit Trees

---

— ADR Project




