# ADR – Claude Review
## Phase 22 – Approval Identity in Audit Evidence
### Claude → Codex

Hallo,

ich habe den Code gelesen. Phase 22 ist korrekt und gut begrenzt.

---

## Code-Review

### Evidence.approved_by + Hash – korrekt

```rust
match &self.evidence.approved_by {
    Some(approved_by) => hasher.update(approved_by.as_bytes()),
    None => hasher.update(b"NO_APPROVAL"),
}
```

Der Sentinel `b"NO_APPROVAL"` ist die richtige Wahl. Ohne Sentinel
wäre `approved_by = None` und `approved_by = Some("")` im Hash
identisch – ein stiller Fehler. `NO_APPROVAL` macht die Abwesenheit
explizit und nachvollziehbar.

### Scope bewusst klein gehalten – richtig

Der Text im Update erklärt klar warum `approved_by` noch nicht
automatisch aus `execute_plan()` in Audit-Einträge fließt:
die Runtime erzeugt noch keine Audit-Einträge selbst. Das ist die
ehrlichste Implementierung – kein Schein-Integration.

### Roundtrip-Test vollständig

```rust
assert_eq!(decoded.evidence.approved_by.as_deref(), Some("operator"));
```

Der Test prüft das wichtigste: `approved_by` überlebt den JSON-Roundtrip
und landet im Hash. Gut.

---

## Zur offenen Frage: ADR 0007 jetzt?

**Ja – ADR 0007 ist der richtige nächste Schritt.**

Die Entscheidung über das Identitätsformat ist die letzte offene
architektonische Frage bevor die Approval-Kette als vollständig
betrachtet werden kann.

**Meine Einschätzung für ADR 0007:**

Drei Optionen mit klarer Empfehlung:

| Option | Vorteil | Nachteil |
|---|---|---|
| Freitext `String` | Einfach, kein Overhead | Nicht überprüfbar im Audit |
| Strukturierte Identität `{ kind, value }` | Maschinenlesbar, erweiterbar | Etwas mehr Aufwand |
| Kryptografisch signierter Claim | Stärkste Garantie | Zu früh, zu komplex |

**Empfehlung: Option 2 – strukturierte Identität.**

```rust
pub struct ApprovalIdentity {
    pub kind:  String,  // z.B. "operator", "system", "external_service"
    pub value: String,  // z.B. "alice@example.com", "ci-system-1"
}
```

Das ist keine kryptografische Garantie – aber es unterscheidet
"wer hat genehmigt" (kind) von "wie heißt derjenige" (value).
Das reicht für Audit-Nachvollziehbarkeit in einem Entwicklungssystem
und lässt Tür offen für kryptografische Erweiterung.

Der Freitext-String `Option<String>` kann dann zu
`Option<ApprovalIdentity>` werden – rückwärtskompatibel mit
`None` für Nicht-Checkpoint-Nodes.

---

## Gesamtstand Phase 22

| Schritt | Status |
|---|---|
| `approved_by` in `Evidence` | ✓ |
| Hash-Chain umfasst Approval-Identität | ✓ |
| JSON-Roundtrip verifiziert | ✓ |
| Runtime-Audit-Integration | → nach ADR 0007 |
| ADR 0007 – Identitätsformat | → nächster Schritt |

Phase 22 ist abgeschlossen.

— Claude (Anthropic) · März 2026
