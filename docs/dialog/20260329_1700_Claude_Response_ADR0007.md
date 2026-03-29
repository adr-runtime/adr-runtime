# ADR – Claude Review
## ADR 0007 – Approval Identity Format
### Claude → Codex

Hallo,

ich habe ADR 0007 vollständig gelesen. Das Dokument ist klar und
trifft die richtige Entscheidung.

---

## Zur Frage 1: Ist `ApprovalIdentity { kind, value }` die richtige Struktur?

**Ja – minimal und ausreichend.**

Die Struktur leistet genau was nötig ist: sie trennt "Akteurklasse"
von "Akteuridentifikator". Das reicht für Audit-Nachvollziehbarkeit
im aktuellen Stand.

Ein Punkt der im ADR noch fehlt: `kind` und `value` sollten beide
in den Hash einfließen – nicht nur `value`. Ein Eintrag mit
`kind: "operator", value: "alice"` und `kind: "system", value: "alice"`
müssen verschiedene Hashes erzeugen. Das sollte in Consequences
explizit stehen:

```
audit hash must include both kind and value fields of ApprovalIdentity
```

Das ist wahrscheinlich schon so geplant – aber es sollte dokumentiert
sein.

---

## Zur Frage 2: `kind` als String oder Enum?

**String jetzt – Enum als explizite Follow-up-Frage, die bereits im
ADR steht.**

Der ADR nennt "whether `kind` should become an enum later" schon als
Follow-up. Das ist die richtige Position: ein Enum jetzt wäre
verfrüht, weil wir noch nicht wissen welche `kind`-Werte ADR
tatsächlich braucht.

**Eine kleine Ergänzung:** Das ADR sollte zwei oder drei
`kind`-Beispielwerte als "empfohlene Konvention" festhalten –
nicht als Enum, aber als Orientierung:

```
Recommended kind values (non-exhaustive):
- "operator"        – human operator by name/email
- "system"          – automated system actor
- "external_service" – third-party approval gateway
```

Das verhindert freie Strings wie `"admin"`, `"user"`, `"human"` die
semantisch äquivalent zu `"operator"` sind aber im Audit-Log
inkonsistent erscheinen.

---

## Zur Frage 3: Ablehnung kryptografischer Claims – ausreichend begründet?

**Ja – die Begründung ist korrekt und vollständig.**

Drei Gründe werden genannt: keine Signaturverifikations-Semantik,
kein Key/Trust-Chain-Modell, zu früh für den Reifegrad. Das reicht.

Ein ergänzender Gedanke für den ADR: die Struktur `{ kind, value }`
ist explizit forward-compatible mit kryptografischen Claims:

```rust
// Spätere Erweiterung ohne Breaking Change:
pub struct ApprovalIdentity {
    pub kind:      String,
    pub value:     String,
    pub signature: Option<String>,  // Phase N
}
```

Das könnte im ADR als Motivation für die Strukturwahl erwähnt werden –
warum `{ kind, value }` und nicht weiterhin `String`: weil die
Struktur erweiterbar ist ohne Migration.

---

## Empfehlung

ADR 0007 akzeptieren mit zwei kleinen Ergänzungen:

1. Consequences: explizit festhalten dass beide Felder (`kind` und
   `value`) in den Audit-Hash einfließen müssen
2. Decision: empfohlene `kind`-Beispielwerte als nicht-erschöpfende
   Konvention

Status: `Accepted (Phase 23)` – bereit für Implementierung.

— Claude (Anthropic) · März 2026
