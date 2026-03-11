# ADR 0001: Layer Boundary

## Status
Accepted

## Kontext

ADR – Agent-Oriented Declarative Runtime – besteht aus mehreren logischen Schichten.

Während der frühen Architekturphase bestand die Gefahr, dass semantische Planung
(Resolver) und sicherheitskritische Ausführung (Runtime) vermischt werden.

Eine klare Trennung ist notwendig, um Safety-Invarianten garantieren zu können.

## Entscheidung

Das System wird in zwei Hauptschichten getrennt:

Layer 1 – Runtime / Safety Kernel  
Layer 2 – Resolver / Planning Layer

Layer 1 enthält:

- RuntimeState
- CapabilitySet
- Executor
- KillSwitch
- Audit Log

Layer 2 enthält:

- IntentResolver
- Policy Matching
- Graph Planung

Resolver-Code läuft **niemals im Runtime-Kern**.

Der Resolver schlägt Pläne vor.  
Der Executor entscheidet, ob sie ausgeführt werden dürfen.

## Begründung

Diese Trennung stellt sicher:

- Resolver-Fehler können den Safety-Kern nicht kompromittieren
- Runtime bleibt deterministisch
- Sicherheitsregeln werden immer im Executor erzwungen

## Konsequenzen

- Resolver darf niemals direkten Zugriff auf RuntimeState haben
- Runtime überprüft alle Sicherheitsbedingungen selbst
- Resolver-Ergebnisse sind nur Vorschläge