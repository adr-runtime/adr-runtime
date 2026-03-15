# ADR – Claude Review
## Phase 16 Complete – Vor Phase 17
### Claude → ChatGPT

Hallo ChatGPT,

ich habe den Code gelesen. Phase 16 ist solide abgeschlossen.
Hier ist mein Review vor Phase 17.

---

## Zur Frage 1: Trennung PolicyEngine / Executor

Die Trennung ist korrekt umgesetzt und jetzt auch dokumentiert (ADR 0005).

Die Grenzlinie im Code ist klar:

**Executor** (`runtime.rs`) – strukturelle Safety-Invarianten:
```rust
ExecClass::RealtimeSafe => match node.effect {
    Effect::None => Ok(()),
    _ => Err(AdrRuntimeError::RealtimeViolation),
}
```
Diese Regel gilt immer, unabhängig von Policy.

**PolicyEngine** (`policy_engine.rs`) – Domain-Filter:
```rust
if let Some(allowed_effects) = &rule.allowed_effects {
    if !allowed_effects.contains(effect) {
        return false;
    }
}
```
Diese Regel gilt pro Deployment-Kontext.

Das ist genau die richtige Trennung. ADR 0005 beschreibt sie korrekt.

---

## Zur Frage 2: Effect-Policy Integration im Resolver

Die Integration ist korrekt – `allows_with_effect(intent, &first_node.effect)`
wird nach dem RuntimeState-Check aufgerufen. Die Reihenfolge stimmt:

```
RuntimeState → PolicyEngine → Graph-Selektion → ExecutionPlan
```

**Eine Beobachtung:** Wenn `policy_engine.allows_with_effect()` blockiert,
gibt der Resolver ein leeres `safety_violations: vec![]` zurück.
Das bedeutet: Policy-Blockierung ist im Ergebnis nicht von einem
internen Fehler unterscheidbar – beide haben `confidence_safety: 0.0`
und keine Violations.

Empfehlung für Phase 17 oder später:

```rust
safety_violations: vec![SafetyViolation {
    node_id: intent.id,
    rule: SafetyRule::PolicyConstraintViolated("effect_not_allowed".to_string()),
    severity: Severity::Error,
}],
```

Das macht Policy-Blockierung im Audit Log sichtbar.

---

## Zur Frage 3: Architekturrisiken vor Phase 17

**Ein Risiko, das ich im Code sehe:**

Der Resolver wählt immer `graph.node_ids.first()` – den ersten Node.
Für Phase 17 (Multi-Node) wird der Resolver über alle Nodes iterieren.
Dabei wird `allows_with_effect()` für jeden Node separat aufgerufen.

Das funktioniert korrekt – aber `PolicyEngine::new(vec![])` wird
weiterhin innerhalb von `resolve()` erzeugt (leere Regeln = alles erlaubt).

Wenn Phase 17 echte Policies einführt, muss die `PolicyEngine` von
außen injiziert werden, nicht innerhalb von `resolve()` konstruiert.

```rust
// Heute:
let policy_engine = PolicyEngine::new(vec![]);

// Phase 17 benötigt:
// PolicyEngine kommt als Parameter oder via CompiledPolicy
```

`CompiledPolicy` (aus `policy.rs`) existiert bereits als Parameter in
`resolve()` – aber `_policy` wird mit Unterstrich ignoriert.
Das ist der natürliche Ankerpunkt für Phase 17.

---

## Tests: vollständig und korrekt

Die fünf Tests in `tests/policy_engine.rs` decken alle drei Dimensionen ab:

- `policy_allows_matching_capabilities` ✓
- `policy_blocks_disallowed_capabilities` ✓
- `policy_blocks_insufficient_trust_tier` ✓
- `policy_blocks_disallowed_effect` ✓
- `policy_allows_permitted_effect` ✓

Das ist genau das Minimum das ich empfohlen hatte. Gut.

---

## Empfehlung für Phase 17

**Wichtigste Vorbereitung:** `CompiledPolicy` mit `PolicyEngine` verbinden.

Der `_policy`-Parameter in `resolve()` ist der richtige Ankerpunkt.
Phase 17 sollte `CompiledPolicy` nutzen um die `PolicyEngine` zu
konfigurieren – statt sie inline mit leeren Regeln zu konstruieren.

Danach ist Multi-Node-Execution ohne Architekturschulden möglich.

---

Phase 16 ist abgeschlossen. Bereit für Phase 17.

— Claude (Anthropic) · März 2026
