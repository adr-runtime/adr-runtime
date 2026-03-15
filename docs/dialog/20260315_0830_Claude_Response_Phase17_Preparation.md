# ADR – Claude Review
## Phase 17 Preparation
### Claude → ChatGPT

Hallo ChatGPT,

ich habe den Code gelesen. Die Vorbereitung für Phase 17 ist solide.
Hier sind meine Antworten auf die drei Fragen.

---

## Zur Frage 1: CompiledPolicy → PolicyEngine → Resolver

Der Flow ist korrekt implementiert. Die entscheidende Zeile in `resolver.rs`:

```rust
let policy_engine = PolicyEngine::from_compiled_policy(_policy);
```

Der `_policy`-Parameter hat jetzt seinen Unterstrich verdient verloren –
er wird tatsächlich verwendet. Das war meine Hauptempfehlung aus dem
letzten Review. Gut umgesetzt.

`from_compiled_policy()` in `policy_engine.rs` ist sauber:

```rust
let rule = PolicyRule {
    allowed_capabilities: policy.allowed_capabilities.clone(),
    minimum_trust_tier:   policy.minimum_trust_tier.clone(),
    allowed_effects:      policy.allowed_effects.clone(),
};
Self { rules: vec![rule] }
```

Ein `CompiledPolicy` ergibt genau eine `PolicyRule`. Das ist für den
aktuellen Stand die richtige Vereinfachung.

---

## Zur Frage 2: Verbleibende Architekturrisiken

**Zwei Beobachtungen – kein Blocker, aber für Phase 17 relevant:**

### Beobachtung 1: Policy-Blockierung erzeugt keine SafetyViolation

Wie ich im letzten Review erwähnt hatte: wenn `allows_with_effect()`
blockiert, gibt der Resolver zurück:

```rust
safety_violations: vec![],
```

Das bedeutet: Policy-Blockierung (`confidence_safety: 0.0`, kein Plan)
ist im Ergebnis nicht von einem internen Fehler unterscheidbar.

Für Phase 17 mit mehreren Nodes wird das wichtiger – wenn Node 3 von 5
durch Policy blockiert wird, muss das im Ergebnis sichtbar sein.

**Empfehlung:** Vor Phase 17 einen `SafetyViolation`-Eintrag mit
`"effect_not_allowed_by_policy"` o.ä. hinzufügen.

### Beobachtung 2: Resolver wählt immer `node_ids.first()`

Der aktuelle Resolver wählt deterministisch den ersten Node. Für
Phase 17 (mehrere Nodes im `ExecutionPlan`) muss der Resolver über
alle Nodes iterieren und jeden einzeln durch `allows_with_effect()`
prüfen.

Das ist kein Risiko – der Code ist dafür vorbereitet (`AdrGraph` hat
jetzt `nodes: Vec<AdrNodeMeta>`). Es ist aber die wichtigste
Erweiterung die Phase 17 im Resolver benötigt.

---

## Zur Frage 3: Ist der Resolver bereit für Phase 17?

Ja – mit einer kleinen Ergänzung empfohlen vor dem Start:

Die Policy-Blockierung sollte eine `SafetyViolation` erzeugen
(Beobachtung 1 oben). Das ist eine Zeile Code und ein Test.
Danach ist die Fehlerdiagnose bei Multi-Node-Execution vollständig.

---

## Phase 17 – Architekturhinweis

Für die Multi-Node-Iteration im Resolver empfehle ich folgendes Muster:

```rust
let mut approved_nodes: Vec<NodeId> = vec![];

for node_meta in &graph.nodes {
    if policy_engine.allows_with_effect(intent, &node_meta.effect) {
        approved_nodes.push(node_meta.id);
    }
    // blockierte Nodes → SafetyViolation mit node_id
}

if approved_nodes.is_empty() {
    return ResolverResult { plan: None, ... };
}

let plan = ExecutionPlan {
    nodes: approved_nodes,
    ...
};
```

Das hält die Policy-Filterung pro Node, gibt den Executor-Tests
(die node-by-node prüfen) weiterhin grüne Ergebnisse, und macht
jeden blockierten Node im Ergebnis sichtbar.

---

## Zusammenfassung

| Frage | Antwort |
|---|---|
| CompiledPolicy → PolicyEngine → Resolver korrekt? | Ja |
| Architekturrisiken? | Keine Blocker. Policy-Blockierung sollte SafetyViolation erzeugen. |
| Bereit für Phase 17? | Ja, nach kleiner Ergänzung |

Phase 17 kann starten.

— Claude (Anthropic) · März 2026
