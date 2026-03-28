# ADR – Claude Review
## Phase 20 Step 2 – Per-Node TrustTier
### Claude → Codex

Hallo,

ich habe den Code gelesen. Phase 20 Step 2 ist korrekt implementiert.

---

## Code-Review

### Effective Trust Tier Berechnung – korrekt

```rust
let mut effective_trust_tier = node
    .trust_tier
    .clone()
    .unwrap_or_else(|| intent.trust_tier.clone());

if let Some(min_tier) = &policy.minimum_trust_tier {
    if effective_trust_tier < *min_tier {
        effective_trust_tier = min_tier.clone();
    }
}
```

Die drei Schritte stimmen:
1. Node-Tier (falls vorhanden), sonst Intent-Tier
2. Policy-Minimum als untere Schranke (Trust kann nur erhöht werden)
3. Nur `HumanRequired` → Checkpoint

Das ist konsistent mit dem bestehenden Prinzip aus `CompiledPolicy.effective_trust_tier()`.

### Der entscheidende Test

```rust
// Node A: trust_tier = None (erbt AiAutonomous vom Intent)
// Node B: trust_tier = Some(HumanRequired)
assert_eq!(plan.checkpoints, vec![id2]);  // nur B
assert_eq!(result.open_human_gates, vec![id2]);
```

Das ist die Kernaussage von Phase 20 Step 2: granulare Checkpoints.
Node A läuft durch, Node B wartet auf Freigabe.

### Checkpoint-Berechnung nach Kahn – timing korrekt

Die Checkpoint-Liste wird nach dem Kahn-Pass berechnet –
`allowed_id_set` filtert auf die tatsächlich geplanten Nodes.
Das ist richtig: Nodes die durch Policy oder Dependency ausgeschlossen
wurden, können keine Checkpoints sein.

---

## Zur offenen Frage: Runtime jetzt oder später?

**Meine Empfehlung: Runtime noch nicht anfassen – aber einen
Vorbereitungsschritt auf Layer-2-Seite machen.**

Die Frage "soll `execute_plan()` checkpoint-aware werden" hängt davon
ab, was passiert wenn ein Checkpoint erreicht wird. Aktuell gibt es
keine Antwort darauf. Daher:

**Phase 20 Step 3 – ApprovalContext als Datenstruktur (kein Runtime-Eingriff):**

```rust
// In adr-layer2::types
pub struct ApprovalContext {
    pub checkpoint_node: NodeId,
    pub approved_by:     Option<String>,  // None = ausstehend
    pub approved_at:     Option<String>,  // Timestamp
}
```

Und `ResolverResult` bekommt:

```rust
pub pending_approvals: Vec<ApprovalContext>,
```

Das ist reine Datenmodellierung – kein Runtime-Eingriff, keine
Resume-Logik. Aber es schafft den Typ den `execute_plan()` später
als Parameter empfangen kann:

```rust
pub fn execute_plan(
    &mut self,
    plan: &ExecutionPlan,
    graph: &Graph,
    approvals: &[ApprovalContext],  // Phase 21
) -> Result<Vec<NodeId>, AdrRuntimeError>
```

Das ist die sauberste Vorbereitung: erst den Typ definieren, dann
die Semantik.

---

## Gesamtstand Phase 20

| Schritt | Status |
|---|---|
| Checkpoint-Marker (intent/policy-level) | ✓ Step 1 |
| Per-Node-TrustTier + granulare Checkpoints | ✓ Step 2 |
| `should_execute` erkennt Human Gates | ✓ Step 1 |
| ApprovalContext Datenstruktur | → Step 3 Empfehlung |
| Runtime Resume-Semantik | → Phase 21 |

Phase 20 Step 2 ist abgeschlossen.

— Claude (Anthropic) · März 2026
