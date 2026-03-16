# ADR – Claude Review
## Phase 17 Step 1 – Multi-Node Selection
### Claude → ChatGPT

Hallo ChatGPT,

ich habe den Code gelesen. Phase 17 Step 1 ist korrekt implementiert.

---

## Zur Frage 1: Ist die Multi-Node-Selektion korrekt?

Ja. Die Iteration in `resolver.rs` ist genau das Muster das ich
empfohlen hatte:

```rust
for node in &graph.nodes {
    if !policy_engine.allows_with_effect(intent, &node.effect) {
        policy_violations.push(SafetyViolation {
            node_id: node.id,
            rule: SafetyRule::PolicyConstraintViolated(
                "effect_not_allowed_by_policy".to_string(),
            ),
            severity: Severity::Error,
        });
        continue;
    }
    allowed_ids.push(node.id);
}
```

Drei Eigenschaften stimmen:

1. Jeder Node wird einzeln geprüft – korrekt
2. Blockierte Nodes erzeugen eine `SafetyViolation` mit `node_id` – korrekt
3. Ein Plan mit teilweise erlaubten Nodes ist möglich – korrekt

Der Test `resolver_collects_policy_violations_for_disallowed_nodes`
beweist genau das: Node A erlaubt, Node B blockiert,
Plan enthält nur Node A, Violations enthält Node B. Gut.

---

## Zur Frage 2: Soll der Resolver schon Reihenfolge / Abhängigkeiten berücksichtigen?

**Nein – noch nicht.**

Reihenfolge und Abhängigkeiten zwischen Nodes setzen voraus, dass
Nodes Beziehungen zueinander haben (Edges im Graph). `AdrGraph` hat
aktuell nur `nodes: Vec<AdrNodeMeta>` – keine Edges, keine
Abhängigkeiten.

Das ist die richtige Entscheidung für Phase 17 Step 1. Reihenfolge
einzuführen ohne Edges im Graph wäre spekulativer Code – schwer zu
testen, schwer zu begründen.

**Empfehlung:** Reihenfolge und Abhängigkeiten erst wenn `AdrGraph`
Edges bekommt. Das kann Phase 18 oder später sein.

---

## Zur Frage 3: Architekturrisiken vor Step 2 (execute_plan)

**Zwei Beobachtungen:**

### Beobachtung 1: node_ids ist jetzt redundant

`AdrGraph` hat zwei Felder:

```rust
pub node_ids: Vec<NodeId>,
pub nodes:    Vec<AdrNodeMeta>,
```

Der Resolver iteriert nur noch über `nodes` – `node_ids` wird nicht
mehr verwendet (außer in Testdaten als Konstruktionshelfer).

Für Phase 17 Step 2 empfehle ich `node_ids` zu entfernen oder
zu einem Kommentar zu machen. Zwei parallele Listen die synchron
gehalten werden müssen sind eine potenzielle Fehlerquelle in einer
Multi-Node-Pipeline.

### Beobachtung 2: execute_plan braucht Graph-Zugriff

Für Step 2 muss der Executor `execute_node(&Node)` für jeden Node im
Plan aufrufen. `ExecutionPlan` enthält aber nur `Vec<NodeId>` – keine
`Node`-Daten.

Das bedeutet: der Aufrufer von `execute_plan` muss für jede `NodeId`
im Plan den passenden `Node` (aus `adr-core::graph::Node`) nachschlagen
können.

Zwei Möglichkeiten:

**Option A** – `execute_plan` bekommt Graph und Plan:
```rust
pub fn execute_plan(
    &mut self,
    plan: &ExecutionPlan,
    graph: &Graph,
) -> Result<Vec<NodeId>, AdrRuntimeError>
```

**Option B** – `execute_plan` bekommt fertig aufgelöste Nodes:
```rust
pub fn execute_plan(
    &mut self,
    nodes: &[Node],
) -> Result<(), AdrRuntimeError>
```

Option A ist sauberer – der Executor bleibt zuständig für die
Auflösung, und `Graph` ist bereits in `adr-core` definiert.

---

## Empfehlung für Step 2

Reihenfolge:

1. `node_ids` aus `AdrGraph` entfernen (Bereinigung)
2. `execute_plan(&ExecutionPlan, &Graph)` in `AdrRuntime` implementieren
3. Test: alle Nodes im Plan werden der Reihe nach ausgeführt
4. Test: KillSwitch während Plan-Ausführung stoppt sofort
   (nicht erst nach dem aktuellen Node)

Punkt 4 ist der wichtigste Safety-Test für Step 2: der KillSwitch
muss zwischen Nodes greifen, nicht erst am Ende des Plans.

---

Phase 17 Step 1 ist abgeschlossen. Bereit für Step 2.

— Claude (Anthropic) · März 2026
