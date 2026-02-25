# ADR – Agent-Oriented Declarative Runtime

> **Layer 1:** deterministische Safety Engine.
> **Layer 2:** erklärbare Entscheidungslogik.

ADR ist eine Safety-Orchestrierungsruntime für KI-Agenten.
Entstanden im offenen Dialog zwischen KI-Systemen verschiedener Anbieter
und einem menschlichen Moderator.

## Warum ADR?
Aktuelle Programmiersprachen sind für Menschen optimiert.
ADR ist für den Betrieb von KI-Agenten in kritischen Umgebungen
optimiert: Medizin, Energie, Raumfahrt, Industriesteuerung.

## Was ADR NICHT ist
ADR ersetzt keine deterministischen Low-Level-Safety-Loops,
Hardware-Interlocks oder physische Sicherheitssysteme.
Siehe [SCOPE.md](./SCOPE.md).

## Status
Spezifikation v0.5 – Proof-of-Concept in Entwicklung.

## Dokumente
| Datei | Inhalt |
|-------|--------|
| [SPEC.md](./SPEC.md) | Vollständige Spezifikation |
| [SCOPE.md](./SCOPE.md) | Was ADR ist und nicht ist |
| [CHANGELOG.md](./CHANGELOG.md) | Versionsgeschichte |
| [docs/dialog/](./docs/dialog/) | Der vollständige Dialog |

## Lizenz
- Core (Rust): Apache-2.0
- Tooling: MIT
- Spezifikation: CC-BY-4.0

---
*Entstanden im offenen Dialog zwischen KI-Systemen verschiedener Anbieter
und einem menschlichen Moderator. Februar 2026.*
