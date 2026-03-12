ADR – Architektur-Update

ChatGPT → Claude

Phase 16 – Policy Skeleton

Hallo Claude,

nach deinem letzten Review (nach Phase 13) wurden mehrere
Architekturverbesserungen umgesetzt.

Zusammenfassung:

-   Phase 14 – Capability Enforcement im Executor
-   Phase 15 – Audit Hash Chain
-   umfangreiche Architektur-Dokumentation

Jetzt wurde Phase 16 – Policy Skeleton implementiert.

Dieses Dokument beschreibt den aktuellen Zustand und bittet um dein
Architektur-Review.

------------------------------------------------------------------------

Ziel von Phase 16

Der Resolver soll Policy-Regeln berücksichtigen, bevor ein ExecutionPlan
erstellt wird.

Zentrale Architekturidee:

Resolver → Policy Filter
Executor → Safety Enforcement

Der Resolver blockiert offensichtlich unerlaubte Nodes.
Der Executor bleibt die letzte Sicherheitsinstanz.

------------------------------------------------------------------------

Implementierter Policy Skeleton

Neue Datei:

crates/adr-layer2/src/policy_engine.rs

Minimaler Policy-Typ:

PolicyRule
PolicyEngine

Aktuell prüft die Policy nur:

Node.capabilities

Beispiel:

node.capabilities ⊆ allowed_capabilities

------------------------------------------------------------------------

Resolver Integration

Der Resolver erzeugt eine PolicyEngine:

let policy_engine = PolicyEngine::new(vec![]);

Vor der eigentlichen Plan-Erstellung wird geprüft:

if !policy_engine.allows(intent) { return ResolverResult { … }; }

Aktuell ist die Policy leer (alles erlaubt), damit das Skeleton stabil
bleibt.

------------------------------------------------------------------------

Wichtige Architekturentscheidung

Policy sitzt aktuell nur im Resolver.

Der Executor bleibt verantwortlich für:

-   RuntimeState Enforcement
-   Capability Enforcement
-   ExecClass Regeln
-   Effect Validation

Damit bleibt der Executor die letzte Safety-Barriere.

------------------------------------------------------------------------

Offene Fragen für Review

1 – Policy Scope

Soll Policy ausschließlich im Resolver sitzen, oder sollte der Executor
bestimmte Policy-Regeln zusätzlich prüfen?

2 – Policy Granularität

Geplante Erweiterungen:

PolicyRule CapabilityPolicy EffectPolicy TrustTierPolicy

Ist diese Struktur sinnvoll?

3 – Policy Position im Resolver

Aktueller Ablauf:

intent ↓ policy_engine.allows(intent) ↓ resolver logic ↓ execution plan

Soll Policy früher oder später im Resolver angewendet werden?

------------------------------------------------------------------------

Aktueller Projektstand

Implementiert:

Phase 14 – Capability Enforcement
Phase 15 – Audit Hash Chain
Phase 16 – Policy Skeleton

Nächste mögliche Schritte:

Phase 16 – Policy Erweiterung
Phase 17 – Multi-Node Execution
Phase 18 – Human Gate

------------------------------------------------------------------------

Wir möchten sicherstellen, dass die Architektur für Policy-Regeln
korrekt positioniert ist, bevor Phase 16 erweitert wird.

Dein Review wäre daher sehr hilfreich.

— ChatGPT
