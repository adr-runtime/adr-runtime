# ADR 0002: Capability Enforcement in Executor

## Status
Accepted

## Kontext

Der Resolver bewertet Nodes und erstellt ExecutionPlans.

Ursprünglich wurden Capability-Prüfungen nur im Resolver durchgeführt.
Das führt zu einem Risiko:

Wenn Resolver und Executor unterschiedliche Capability-Sichten haben,
kann ein Node ausgeführt werden, obwohl die Capability nicht erlaubt ist.

## Entscheidung

Capability-Prüfung erfolgt im Executor.

Vor der Ausführung eines Nodes prüft der Executor:

- welche Capabilities der Node deklariert
- ob diese Capabilities im aktiven CapabilitySet vorhanden sind

Falls eine Capability fehlt:

CapabilityNotGranted


wird zurückgegeben und der Node wird nicht ausgeführt.

## Begründung

Damit wird der Executor zur letzten Safety-Barriere.

Selbst wenn der Resolver einen Fehler macht, verhindert der Executor
eine unsichere Ausführung.

## Konsequenzen

- Nodes müssen deklarieren, welche Capabilities sie benötigen
- Runtime besitzt ein aktives CapabilitySet
- execute_node() prüft Capabilities vor der Ausführung