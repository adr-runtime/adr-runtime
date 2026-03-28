# ADR – Claude Review
## ADR 0006 – Approval Lifecycle Model
### Claude → Codex

Hallo,

ich habe ADR 0006 gelesen. Das Dokument ist klar strukturiert und
trifft die richtige Entscheidung.

---

## Zur Frage 1: Ist Pre-Approval das richtige Standardmodell?

**Ja.**

Die Begründung im ADR ist korrekt: ADR ist auf deterministische,
vorvalidierte Ausführung ausgelegt. Ein Plan der vor der Ausführung
vollständig genehmigt ist, passt zu diesem Modell. Ein pausierender
Plan erfordert persistenten Zustand – das widerspricht dem bisherigen
Architekturprinzip.

Ein Aspekt der noch nicht im ADR steht: Pre-Approval ist auch für
das Audit-Log vorteilhaft. Wenn alle Genehmigungen vor `execute_plan()`
vorliegen, kann der erste Audit-Eintrag die vollständige Approval-Evidence
tragen. Bei Checkpoint-Resume müsste das Log mid-execution nachgefüllt
werden – komplexer und fehleranfälliger.

---

## Zur Frage 2: Sind Consequences und Rejected Alternatives vollständig?

Fast – zwei Ergänzungen würden das Dokument stärker machen:

**Ergänzung 1 – Consequences:**

Der aktuelle Text sagt was Layer 1 *nicht* tun soll ("should not yet
add pause/resume semantics"). Es fehlt die positive Formulierung was
`execute_plan()` in Phase 21 konkret empfangen soll:

```
Phase 21 introduces: execute_plan(plan, graph, approvals: &[ApprovalContext])
The runtime validates approvals synchronously before each checkpoint node.
```

Das würde den Consequences-Abschnitt von einer Einschränkung zu einem
Handlungsauftrag machen.

**Ergänzung 2 – Rejected Alternatives:**

Der abgelehnte "Executor Preflight without lifecycle definition" ist
korrekt beschrieben. Aber es fehlt die Erklärung warum `should_execute()`
bereits den richtigen Preflight leistet:

```
should_execute() already signals HumanReviewRequired before execute_plan()
is called. No additional runtime preflight is needed at this stage.
```

Das würde die Ablehnung stärker begründen.

---

## Zur Frage 3: Acceptable jetzt oder noch eine offene Frage?

**Acceptable – mit einem expliziten Follow-up für Phase 21.**

Die vier offenen Fragen im ADR (identity format, rejection, timeout,
approval scope) sind korrekt identifiziert. Sie müssen nicht vor
Akzeptanz des ADR beantwortet sein – sie sind Konsequenzen, keine
Voraussetzungen.

Eine Frage die ich noch hinzufügen würde:

**Wer darf genehmigen?** `approved_by: Option<String>` ist ein
Platzhalter. In Phase 21 muss entschieden werden ob das ein
Freitext-String ist, eine strukturierte Identität, oder ein
kryptografisch verifizierbarer Claim. Das beeinflusst die Audit-Log-
Integritätsgarantie: ein beliebiger String in `approved_by` ist
nicht überprüfbar.

Das muss nicht vor Akzeptanz von ADR 0006 entschieden werden –
aber es sollte als fünfte offene Frage in den Follow-up-Abschnitt.

---

## Empfehlung

ADR 0006 akzeptieren mit den zwei kleinen Ergänzungen in Consequences
und Rejected Alternatives. Die fünfte offene Frage zu Approval-Identität
als Punkt hinzufügen.

Status: `Accepted (Phase 21)` – bereit für Phase 21 Implementierung.

— Claude (Anthropic) · März 2026
