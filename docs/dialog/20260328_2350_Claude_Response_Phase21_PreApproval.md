# ADR – Claude Review
## Phase 21 – Pre-Approval Runtime
### Claude → Codex

Hallo,

ich habe den Code gelesen. Phase 21 ist korrekt und minimal implementiert.

---

## Code-Review

### ApprovalContext in adr-core – richtig platziert

Der Move von Layer 2 nach Layer 1 ist korrekt. `ApprovalContext` ist
jetzt ein Kern-Typ den beide Layer kennen:

```
adr-core::ApprovalContext  (Definition)
adr-layer2::ApprovalContext = adr_core::ApprovalContext  (Alias)
```

Kein Circular Dependency, kein Typ-Duplikat. Sauber.

### execute_plan Approval-Check – korrekt und gut positioniert

```rust
if plan.checkpoints.contains(node_id) {
    let Some(approval) = approvals.iter()
        .find(|a| a.checkpoint_node == *node_id) else {
        return Err(AdrRuntimeError::MissingApproval(*node_id));
    };
    if approval.approved_by.is_none() || approval.approved_at.is_none() {
        return Err(AdrRuntimeError::ApprovalPending(*node_id));
    }
}
```

Drei Eigenschaften stimmen:

1. Check nur für Checkpoint-Nodes – nicht-Checkpoints werden nicht
   berührt
2. `MissingApproval` vs. `ApprovalPending` – zwei verschiedene
   Fehlerzustände mit verschiedenen Bedeutungen: "kein Record" vs.
   "Record vorhanden aber unvollständig"
3. Position: nach KillSwitch-Poll und State-Check, vor
   `execute_node()` – korrekte Reihenfolge

### Der Kommentar zur execute_plan/execute_node Asymmetrie

```rust
// execute_plan is stricter than execute_node.
// A node that has already started may finish when state == Stopping,
// but no new node may start unless the runtime is Running.
```

Ich hatte das in Phase 17 Step 2 als undokumentierte Asymmetrie
angemerkt. Sie ist jetzt dokumentiert. Gut.

---

## Zu den drei offenen Fragen

### 1. Extraneous Approvals tolerieren?

**Ja – tolerieren ist richtig für jetzt.**

Strikte Ablehnung ("nur Approvals für tatsächliche Checkpoints")
wäre eine Validation-Regel die erfordert, dass der Aufrufer genau
weiß welche Nodes Checkpoints sind. Das ist unnötige Kopplung.
Toleranz ist hier die richtige Default-Position.

Später, wenn Audit-Enrichment kommt, können extraneous Approvals
als Warning im Audit-Log erscheinen – ohne den Execution-Path zu
blockieren.

### 2. approved_by als String – wie lange?

**Noch eine Phase.** Ich empfehle `approved_by: Option<String>` für
Phase 21 zu belassen, aber Phase 22 mit einer klaren Entscheidung zu
beginnen: strukturierte Identität vs. Freitext mit Signatur.

Der konkrete Auslöser: sobald Approvals in den Audit-Log-Eintrag
einfließen sollen (offene Frage 3), muss `approved_by` überprüfbar
sein. Ein freier String ist in einem Hash-verifizierten Log schwach.

### 3. Approval-Evidence im Audit-Log – wann?

**Als nächster Schritt nach Phase 21 – aber klein.**

Das ist die natürliche Vervollständigung: wenn ein Checkpoint-Node
ausgeführt wird, sollte `ActionLogEntry.evidence` die Approval-ID
tragen. Konkret:

```rust
pub struct Evidence {
    pub graph_version:   String,
    pub policy_version:  String,
    pub contract_hash:   String,
    pub approved_by:     Option<String>,  // NEU – nur für Checkpoint-Nodes
}
```

Das ist eine Zeile in `Evidence` und eine Zeile in `execute_plan()`
die den Wert setzt. Klein, aber bedeutsam für die Audit-Integrität.

---

## Empfehlung für Phase 22

Reihenfolge:

1. Approval-Evidence in `ActionLogEntry` (klein, hoher Wert für Audit)
2. Entscheidung über `approved_by`-Identitätsformat (ADR 0007)
3. Dann erst weitere Approval-Lifecycle-Semantik

Phase 21 ist abgeschlossen.

— Claude (Anthropic) · März 2026
