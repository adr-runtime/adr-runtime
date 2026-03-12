# ADR – Threat Model

Dieses Dokument beschreibt mögliche Risiken und Angriffsvektoren
für ADR – Agent-Oriented Declarative Runtime – sowie die
architektonischen Mechanismen, die diese Risiken begrenzen.

Das Ziel ist nicht, alle Fehler zu verhindern, sondern sicherzustellen,
dass das System selbst bei Fehlverhalten innerhalb definierter
Sicherheitsgrenzen bleibt.

---

# Grundannahmen

ADR läuft typischerweise in einem Umfeld mit:

- autonomen oder semi-autonomen Agenten
- externen Systemen oder APIs
- komplexen Entscheidungslogiken

Agenten können:

- falsche Entscheidungen treffen
- fehlerhafte Pläne erzeugen
- unerwartete Aktionen versuchen

Die Runtime muss daher sicherstellen, dass solche Situationen
keinen unkontrollierten Schaden verursachen.

---

# Threat Categories

Die wichtigsten Bedrohungskategorien sind:

1. Resolver-Fehler
2. Capability Escalation
3. Unsafe Effects
4. Runtime Escape
5. Audit Log Manipulation
6. Runaway Agent Behavior

---

# Resolver Failure

## Beschreibung

Der Resolver erzeugt einen falschen oder unsicheren ExecutionPlan.

Beispiele:

- falsche Node-Auswahl
- falsche Capability-Annahmen
- Policy-Fehler

## Risiko

Ein fehlerhafter Resolver könnte versuchen,
eine unsichere Operation auszuführen.

## Mitigation

ADR trennt Resolver und Runtime.

Der Resolver schlägt nur einen Plan vor.

Der Executor prüft selbst:

- RuntimeState
- Capabilities
- Execution Class
- Effects

Dadurch kann ein Resolver-Fehler nicht direkt
zu einer unsicheren Ausführung führen.

---

# Capability Escalation

## Beschreibung

Ein Node versucht eine Fähigkeit zu nutzen,
die ihm nicht erlaubt ist.

Beispiele:

- Schreiben ins Dateisystem
- externer Netzwerkzugriff
- Hardwaresteuerung

## Risiko

Ein Agent könnte versuchen,
sich mehr Rechte zu verschaffen.

## Mitigation

Der Executor erzwingt Capability-Prüfung:

```
node.capabilities ⊆ runtime.capability_set
```

Fehlende Fähigkeiten führen zu `CapabilityNotGranted`. Die Operation wird nicht ausgeführt.

---

# Unsafe Effects

## Beschreibung

Ein Node versucht einen Effekt auszuführen,
der in seinem Kontext nicht erlaubt ist.

Beispiel: `RealtimeSafe + Effect::FsWrite`

## Risiko

Nicht-deterministische Effekte könnten
Realtime-Operationen destabilisieren.

## Mitigation

ExecClass-Regeln verhindern solche Kombinationen.

RealtimeSafe Nodes dürfen nur `Effect::None` verwenden.

---

# Runtime Escape

## Beschreibung

Ein Agent versucht, die Kontrolle über die Runtime
oder deren Sicherheitsmechanismen zu umgehen.

Beispiele:

- direkte Hardwarezugriffe
- Umgehung der Runtime-Checks

## Mitigation

ADR kontrolliert nur **ob** Aktionen erlaubt sind.

Die eigentliche Ausführung erfolgt in
externen deterministischen Systemen.

Die Runtime bleibt daher klein und kontrollierbar.

---

# Audit Log Manipulation

## Beschreibung

Ein Angreifer versucht, das Audit Log zu verändern,
um Aktionen zu verschleiern.

## Mitigation

Audit-Einträge sind verkettet:

```
entry_hash = SHA256(entry_fields + prev_hash)
```

Jede Änderung eines Eintrags bricht die Hash-Kette.

Manipulation wird sichtbar.

---

# Runaway Agent Behavior

## Beschreibung

Ein Agent gerät in eine Schleife
oder führt extrem viele Aktionen aus.

Beispiele:

- endlose Planung
- wiederholte Aktionen
- unkontrollierte Agent-Strategien

## Mitigation

Mehrere Mechanismen können dies begrenzen:

- RuntimeState
- KillSwitch
- Policy-Limits
- zukünftige Human Gates

Der KillSwitch kann jederzeit
die Runtime stoppen.

---

# Residual Risk

ADR kann nicht verhindern:

- schlechte Entscheidungen eines Agents
- falsche Policies
- fehlerhafte externe Systeme

ADR stellt jedoch sicher, dass:

- Aktionen überprüft werden
- Fähigkeiten kontrolliert werden
- Ereignisse nachvollziehbar bleiben
- das System jederzeit gestoppt werden kann

---

# Zusammenfassung

Das Threat Model basiert auf vier Grundprinzipien:

1. strikte Trennung von Planung und Ausführung
2. Runtime-Enforcement von Sicherheitsregeln
3. kontrollierte Capabilities
4. unveränderliche Audit-Protokolle

Diese Architektur reduziert die Auswirkungen
von Fehlern oder Angriffen erheblich.

---

— ADR Project



