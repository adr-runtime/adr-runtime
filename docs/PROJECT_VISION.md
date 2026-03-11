# ADR – Project Vision

ADR steht für **Agent-Oriented Declarative Runtime**.

Das Projekt untersucht eine grundlegende Frage:

**Wie können autonome Software-Agenten sicher ausgeführt werden?**

Moderne KI-Agenten können komplexe Entscheidungen treffen,
aber ihre Ausführung wird oft von Systemen gesteuert,
die ursprünglich nicht für autonome Entscheidungslogik gebaut wurden.

ADR versucht eine andere Architektur.

---

# Das Problem

Viele aktuelle Agent-Frameworks konzentrieren sich auf:

- Planung
- Reasoning
- Tool-Integration
- Automatisierung

Die tatsächliche **Ausführung von Aktionen** wird dabei oft
als technisches Detail behandelt.

Das führt zu Problemen:

- Agenten können unerwartete Aktionen ausführen
- Sicherheitsregeln sind schwer durchzusetzen
- Systeme werden schwer kontrollierbar
- Logs sind oft unvollständig

Wenn Agenten komplexer werden,
steigt dieses Risiko.

---

# Die zentrale Idee

ADR trennt zwei Aufgaben strikt voneinander:

Planung
und
Ausführung


Der Agent oder Resolver erstellt nur einen **Vorschlag**.

Die Runtime entscheidet:

Darf diese Aktion ausgeführt werden?


Diese Architektur ähnelt bekannten Sicherheitsprinzipien:

- Betriebssystem-Kernel
- Datenbank-Transaction-Engines
- Sandbox-Systeme

Die Runtime fungiert als **Safety Kernel**.

---

# Architekturprinzipien

ADR basiert auf einigen grundlegenden Prinzipien.

## 1. Trennung von Planung und Ausführung

Resolver erzeugen **ExecutionPlans**.

Die Runtime prüft:

- RuntimeState
- Capabilities
- ExecutionClass
- Effects

Erst danach wird eine Aktion ausgeführt.

---

## 2. Runtime-Enforcement

Sicherheitsregeln werden nicht nur beschrieben,
sondern **im Runtime-Kernel erzwungen**.

Beispiele:

- Capability Enforcement
- RuntimeState Checks
- ExecClass Regeln

Damit bleiben Sicherheitsregeln gültig,
selbst wenn der Resolver Fehler macht.

---

## 3. Nachvollziehbarkeit

Alle Aktionen werden protokolliert.

Das Audit Log bildet eine Hash-Kette.


entry1
entry2 → hash(entry1)
entry3 → hash(entry2)


Damit wird Manipulation sichtbar.

---

## 4. Kontrollierte Agent-Fähigkeiten

Agenten erhalten keine unbegrenzten Rechte.

Stattdessen:

node.capabilities ⊆&sube; runtime.capability_set


Nur erlaubte Fähigkeiten können genutzt werden.

---

# Was ADR nicht ist

ADR ist **kein Agent-Framework**.

Es ersetzt nicht:

- LLM-Reasoning
- Planungssysteme
- Tool-Integration

ADR ist eine **Runtime-Schicht**, die sicherstellt,
dass Agenten innerhalb definierter Grenzen operieren.

---

# Ziel des Projekts

ADR soll eine experimentelle Architektur sein,
die zeigt, wie sichere Agent-Runtimes aufgebaut werden können.

Das Projekt verfolgt drei Ziele:

1. sichere Agent-Ausführung
2. nachvollziehbare Systementscheidungen
3. klare Architekturgrenzen

ADR kann später:

- in bestehende Agent-Frameworks integriert werden
- als Referenzarchitektur dienen
- oder als Grundlage für neue Runtime-Systeme genutzt werden.

---

# Entwicklungsmodell

Das Projekt entsteht durch eine Kombination aus:

- Implementierung
- Architektur-Diskussion
- Architecture Decision Records (ADR)

Diskussionen werden dokumentiert in: docs/dialog/


Architekturentscheidungen werden festgehalten in: docs/adr/


---

# Langfristige Vision

Langfristig könnte ADR helfen,
Agent-Systeme kontrollierbarer zu machen.

Besonders wichtig wird dies,
wenn autonome Systeme komplexere Aufgaben übernehmen.

ADR untersucht daher,
wie ein **Safety-Kernel für Agent-Systeme**
aussehen kann.

---

— ADR Project
