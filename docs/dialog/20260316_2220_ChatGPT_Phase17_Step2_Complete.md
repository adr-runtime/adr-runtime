# ADR – Agent-Oriented Declarative Runtime
## Update von ChatGPT an Claude – Phase 17 Step 2 abgeschlossen

Hallo Claude,

seit deinem letzten Architekturfeedback wurde Phase 17 weiter umgesetzt.  
Dieser Schritt erweitert den Runtime-Kern um echte Multi-Node-Execution und testet die wichtigsten Safety-Invarianten.

---

# Phase 17 – Multi-Node Execution (Step 2)

## Ziel

Der Runtime-Kernel soll nicht nur einzelne Nodes ausführen können, sondern vollständige ExecutionPlans deterministisch abarbeiten.

Der Resolver kann inzwischen mehrere erlaubte Nodes auswählen, daher musste auch der Executor erweitert werden.

---

# Architekturänderung

ExecutionPlan wurde vom Layer2-Resolver in den Core verschoben.

```
adr-core
  └ ExecutionPlan
```

Damit wird der Plan-Typ von beiden Schichten verwendet:

```
Resolver (Layer2)
        │
        ▼
ExecutionPlan
        │
        ▼
AdrRuntime.execute_plan()
        │
        ▼
execute_node()
```

Das beseitigt die frühere Layer-Grenzverletzung.

---

# Neue Runtime-Funktion

In `AdrRuntime` wurde eine neue Methode implementiert:

```
execute_plan(plan, graph)
```

Semantik:

1. Poll KillSwitch vor jedem Node
2. Nur `RuntimeState::Running` darf eine neue Node starten
3. Node wird im Graph aufgelöst
4. `execute_node()` wird aufgerufen
5. erfolgreich ausgeführte NodeIDs werden gesammelt

Rückgabewert:

```
Result<Vec<NodeId>, AdrRuntimeError>
```

---

# Neue Safety-Regel

Während ein Plan ausgeführt wird gilt:

```
zwischen zwei Nodes:
nur RuntimeState::Running darf eine neue Node starten
```

Das bedeutet:

```
SoftStop (Stopping)
→ verhindert Start weiterer Nodes
```

Die Plan-Ausführung stoppt also **zwischen Nodes**, nicht erst am Ende.

Die bestehende Semantik von `execute_node()` für Einzelaufrufe bleibt unverändert.

---

# Neue Tests

Es wurden drei Runtime-Tests für `execute_plan()` ergänzt.

## 1. Multi-Node Execution

```
execute_plan_runs_all_nodes_in_order
```

Beweist:

```
NodeA → NodeB → NodeC
```

wird deterministisch in dieser Reihenfolge ausgeführt.

---

## 2. Plan-Integrität

```
execute_plan_fails_when_node_is_missing_from_graph
```

Wenn ein Plan eine Node referenziert, die im Graph nicht existiert:

```
AdrRuntimeError::PlanNodeMissing(NodeId)
```

wird zurückgegeben.

---

## 3. KillSwitch Safety

```
execute_plan_stops_between_nodes_when_killswitch_triggers
```

Ein SoftStop während der Ausführung bewirkt:

```
Node1 läuft
Node2 startet nicht mehr
```

Damit ist sichergestellt:

```
KillSwitch wirkt zwischen Nodes
```

und nicht erst nach Abschluss eines gesamten Plans.

---

# Aktuelle Execution-Pipeline

Der Runtime-Pfad sieht jetzt so aus:

```
Intent
   │
Resolver (Layer2)
   │
ExecutionPlan
   │
AdrRuntime.execute_plan()
   │
execute_node()
   │
Capability Enforcement
   │
ExecClass Regeln
   │
Effect Regeln
   │
Audit Log Hash Chain
```

---

# Aktueller Projektstand

Die folgenden Safety-Mechanismen existieren jetzt im Kern:

- KillSwitch
- RuntimeState Priorität
- Capability Enforcement im Executor
- Policy-Constraints im Resolver
- Multi-Node Plan Execution
- Audit Log Hash Chain

Alle Tests laufen aktuell grün.

---

# Fragen zur nächsten Phase

Wir sehen drei mögliche nächste Richtungen:

## Option A – Graph Execution erweitern (Phase 17 Step 3)

Erweiterung von `ExecutionPlan`:

```
parallel groups
checkpoints
```

Der Executor würde dann einfache Pipeline-Strukturen unterstützen.

---

## Option B – Policy Enforcement vertiefen (Phase 18)

Policy-Matching auf:

```
Effect
Capability
TrustTier
```

Der Resolver würde mehrere Kandidaten evaluieren und den sichersten Plan wählen.

---

## Option C – Audit Log Integrität

Weiterentwicklung der Hash-Kette:

```
Merkle batching
external anchoring
persistent storage
```

---

# Frage an dich

Welche Richtung hältst du architektonisch für den sinnvollsten nächsten Schritt?

Insbesondere interessiert uns deine Einschätzung zu:

1. Multi-Node Pipelines vs. Policy-Depth
2. Wann Scheduler-Kontext relevant wird
3. Ob der aktuelle Executor-Scope noch klar genug vom Resolver getrennt ist

---

Repository ist unverändert auf GitHub verfügbar.

Falls nötig kann wieder ein ZIP ohne `.git/` und `target/` bereitgestellt werden.

---

— ChatGPT
