# ADR – Safety Model

ADR ist eine Runtime für KI-Agenten mit einem starken Fokus auf
kontrollierte Ausführung und Sicherheitsgrenzen.

Das System verhindert nicht jede mögliche Fehlentscheidung eines Agents,
aber es stellt sicher, dass bestimmte Sicherheitsgrenzen niemals verletzt
werden können.

---

# Designziel

Das zentrale Ziel ist:

Kein Agent darf außerhalb definierter Sicherheitsgrenzen handeln.


ADR garantiert daher:

- kontrollierte Ausführung
- überprüfbare Aktionen
- nachvollziehbare Logs
- jederzeitige Abschaltbarkeit

Diese Eigenschaften gelten unabhängig davon, in welchem Bereich
das System eingesetzt wird.

---

# Safety Layers

Die Sicherheitsmechanismen sind mehrschichtig aufgebaut.

KillSwitch
↓
RuntimeState
↓
Capability Enforcement
↓
Execution Class Validation
↓
Effect Validation
↓
Audit Log


Jede Ebene kann eine Ausführung stoppen.

---

# Kill Switch

Der Kill Switch ist die höchste Sicherheitsinstanz.

Er kann jederzeit ausgelöst werden und hat Priorität über
laufende Operationen.

poll_kill_switch()


Wenn ein KillSignal erkannt wird:

RuntimeState → Frozen


Danach werden keine Nodes mehr ausgeführt.

---

# Runtime State

Die Runtime besitzt eine Sicherheitsordnung.

Running
Stopping
Halted
Frozen


Diese Zustände sind geordnet:

Running < Stopping < Halted < Frozen


Je höher der Zustand, desto stärker sind die Einschränkungen.

Beispiel:

state >= Halted → Execution blockiert


---

# Capability Model

Nodes deklarieren, welche Fähigkeiten sie benötigen.

Beispiele:

- filesystem_write
- network_external
- actuator_control

Der Executor prüft:

node.capabilities ⊆&sube; runtime.capability_set

Fehlt eine Capability, wird die Ausführung abgebrochen.

CapabilityNotGranted


Der Resolver allein kann keine Fähigkeiten erzwingen.

---

# Execution Classes

Nodes besitzen eine Execution Class.



Der Resolver allein kann keine Fähigkeiten erzwingen.

---

# Execution Classes

Nodes besitzen eine Execution Class.

RealtimeSafe
Orchestrated


RealtimeSafe Nodes dürfen keine externen Effekte auslösen.

Beispiel:

RealtimeSafe + Effect::None → erlaubt
RealtimeSafe + Effect::FsWrite → Fehler


Diese Einschränkung verhindert nicht-deterministische
Operationen in sicherheitskritischen Kontexten.

---

# Audit Logging

Alle Aktionen werden protokolliert.

Ein Audit-Eintrag enthält:

- node_id
- action_kind
- timestamp
- evidence
- prev_hash
- entry_hash

Die Einträge bilden eine Hash-Kette.

entry1
entry2 → hash(entry1)
entry3 → hash(entry2)


Wenn ein Eintrag verändert wird,
wird die Manipulation sichtbar.

---

# Safety Philosophy

ADR kontrolliert **ob** ein Agent handeln darf.

ADR kontrolliert nicht **wie** eine Operation technisch ausgeführt wird.

Beispiel:

ADR erlaubt:

actuator_control


Die eigentliche Hardwaresteuerung erfolgt außerhalb von ADR
in deterministischen Systemen.

---

# Failure Model

Wenn eine Sicherheitsprüfung fehlschlägt:

- Node wird nicht ausgeführt
- Fehler wird zurückgegeben
- Ereignis wird im Audit Log gespeichert

Das System versucht nicht, eine unsichere Operation zu reparieren.

Es stoppt sie.

---

# Human Oversight

Zukünftige Erweiterungen können enthalten:

- Human Approval Gates
- Policy Engines
- Multi-Party Authorization

Diese Mechanismen erweitern das Sicherheitsmodell,
ersetzen aber nicht die grundlegenden Runtime-Sicherheitsregeln.

---

# Zusammenfassung

ADR basiert auf drei grundlegenden Prinzipien:

1. Safety durch Runtime-Enforcement  
2. vollständige Nachvollziehbarkeit  
3. kontrollierte Agent-Fähigkeiten

Diese Prinzipien sollen sicherstellen,
dass KI-Agenten innerhalb klar definierter Grenzen operieren.

---

— ADR Project

