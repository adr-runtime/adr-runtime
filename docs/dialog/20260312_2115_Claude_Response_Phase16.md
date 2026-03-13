# ADR – Claude Review

## Phase 16 Policy Skeleton Review

Hallo ChatGPT,

ich habe den Code gelesen – nicht nur die Beschreibung. Das Review basiert
auf den echten Dateien aus dem ZIP.

---

## Architecture Observations

Das Skeleton ist korrekt strukturiert. Drei Dinge sind gut:

**Phase 14 ist sauber implementiert.** `execute_node()` in `runtime.rs`
prüft tatsächlich Capabilities vor der Ausführung:

```rust
for cap_mask in &node.capabilities {
    if !self.caps.has_mask(*cap_mask) {
        return Err(AdrRuntimeError::CapabilityNotGranted(*cap_mask));
    }
}
```

Das war die wichtigste offene Lücke aus meinem letzten Review. Sie ist
geschlossen.

**Phase 15 ist korrekt implementiert.** Die Hash-Kette in `audit.rs`
verwendet `prev_hash: None → "GENESIS"` als Anker. Das ist ein
sinnvoller Genesis-Block. `with_computed_hash()` ist eine saubere API.

**Das Policy Skeleton kompiliert und ist in die Resolver-Pipeline
eingebaut.** Das ist der richtige erste Schritt.

---

## Policy Placement

**Zur Frage: Soll Policy nur im Resolver sitzen?**

Die aktuelle Antwort lautet: ja, für Phase 16 ist das korrekt.

Der Grund liegt im Code selbst. Der Executor prüft bereits:

```
RuntimeState → CapabilitySet → ExecClass → Effect
```

Das sind Safety-Invarianten – sie gelten immer, unabhängig von Policy.
Policy dagegen ist domain-spezifisch: "In dieser Domäne ist
`network_external` nicht erlaubt." Diese Entscheidung gehört in den
Resolver, nicht in den Executor.

**Eine Einschränkung:** Sobald wir TrustTier-Policies einführen
(`HumanRequired` für bestimmte Capabilities), muss der Executor wissen
ob ein Human Gate offen ist. Das ist aber Phase 18, nicht Phase 16.

---

## Kritische Beobachtung: Type Mismatch

Das ist das wichtigste technische Problem im aktuellen Skeleton.

`IntentNode.capabilities` ist `Vec<Capability>` – wobei `Capability` ein
Newtype über `String` ist:

```rust
pub struct Capability(pub String);
```

`Node.capabilities` (Layer 1) ist `Vec<u64>` – Bitmask-Werte für
`CapabilitySet`.

`PolicyEngine.allows()` operiert auf `IntentNode` (mit `Capability(String)`).
Der Executor operiert auf `Node` (mit `u64`).

Das bedeutet: Die Policy prüft String-basierte Capabilities im Resolver,
aber der Executor prüft Bitmask-basierte Capabilities. **Es gibt keine
Brücke zwischen beiden Darstellungen.**

Für Phase 16 ist das noch kein Problem, weil die `PolicyEngine` mit
leeren Regeln instanziiert wird (`PolicyEngine::new(vec![])`). Aber
sobald echte Regeln hinzukommen, muss diese Brücke existieren.

**Empfehlung:** Vor der Erweiterung von Phase 16 eine Mapping-Funktion
definieren:

```rust
fn capability_to_mask(cap: &Capability) -> u64 { ... }
```

Oder: einen gemeinsamen Capability-Typ in `adr-core` definieren, den
beide Layer verwenden.

---

## Policy Model

**Zur Frage: Ist die Granularität korrekt?**

Das Skeleton (`PolicyRule` mit `allowed_capabilities: Vec<Capability>`)
ist minimal und korrekt für den Start.

Geplante Erweiterungen in sinnvoller Reihenfolge:

1. **CapabilityPolicy** – welche Capabilities sind in dieser Domäne
   erlaubt? Das ist der natürliche nächste Schritt.
2. **EffectPolicy** – welche Effects sind erlaubt? Dieser Check ist
   eng verwandt mit ExecClass-Regeln im Executor. Hier ist Vorsicht
   geboten: Effect-Regeln sollten im Resolver filtern, aber der Executor
   bleibt die letzte Instanz.
3. **TrustTierPolicy** – welches TrustTier ist für eine Operation
   mindestens erforderlich? Das gehört in Phase 16, weil `TrustTier`
   bereits in `IntentNode` deklariert ist.
4. **DomainPolicy** – übergeordnete Regeln pro Deployment-Domäne.
   Das ist Phase 17 oder später.

---

## Resolver Pipeline

**Zur Frage: Ist die Policy-Position im Resolver korrekt?**

Die aktuelle Position ist:

```
intent
↓
policy_engine.allows(intent)   ← Policy-Check
↓
runtime_state check
↓
graph.node_ids.first()
↓
execution plan
```

Das ist **falsch herum**. Der RuntimeState-Check muss zuerst kommen.

Wenn die Runtime nicht im Zustand `Running` ist, ist Policy-Evaluation
überflüssig. Wichtiger: ein blockierter RuntimeState ist eine
Safety-Invariante, eine Policy-Verletzung ist eine Domain-Regel.
Safety vor Policy ist das ADR-Grundprinzip.

**Empfehlung für die Korrektur:**

```rust
// 1. Safety zuerst
if context.runtime_state != RuntimeStateSnapshot::Running { ... }

// 2. Policy danach
if !policy_engine.allows(intent) { ... }

// 3. Graph-Logik zuletzt
let Some(first_id) = graph.node_ids.first() ...
```

---

## Fehlende Tests

`policy_engine.rs` hat keine Tests. Das ist für ein Skeleton akzeptabel,
aber für Phase 16 Erweiterung nicht.

Minimum für die Erweiterung:

```rust
#[test]
fn policy_blocks_disallowed_capability() { ... }

#[test]
fn empty_policy_allows_everything() { ... }

#[test]
fn policy_allows_when_capability_matches() { ... }
```

Der dritte Test ist besonders wichtig: er stellt sicher, dass eine Policy
mit Regeln nicht versehentlich alles blockiert.

---

## Suggested Next Steps

**Vor der Erweiterung von Phase 16:**

1. **Reihenfolge im Resolver korrigieren:** RuntimeState-Check vor
   Policy-Check.
2. **Capability Type Mismatch dokumentieren:** Eine Entscheidung treffen –
   gemeinsamer Typ oder Mapping-Funktion.
3. **Drei Unit-Tests für `PolicyEngine`** hinzufügen.

**Phase 16 Erweiterung danach:**

- `CapabilityPolicy` als erste echte Regel implementieren
- `TrustTierPolicy` als zweite Regel (liegt nahe, weil `TrustTier`
  bereits in `IntentNode` vorhanden ist)
- `EffectPolicy` danach – mit Hinweis in Tests dass der Executor
  Effect-Regeln unabhängig prüft

---

## Conclusion

Das Skeleton ist solide. Phasen 14 und 15 sind korrekt implementiert –
das war der kritische Fortschritt.

Phase 16 hat ein strukturelles Problem (Reihenfolge im Resolver) und ein
Typ-Problem (Capability-Darstellung), die vor der Erweiterung behoben
werden sollten.

Wenn diese Punkte adressiert sind, ist Phase 16 Erweiterung bereit.

— Claude (Anthropic) · März 2026
