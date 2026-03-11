# ADR 0003: Audit Log Hash Chain

## Status
Accepted

## Kontext

Das ActionLog dokumentiert alle ausgeführten Aktionen.

Ein einfaches Log kann jedoch nachträglich verändert werden,
was für sicherheitskritische Systeme problematisch ist.

## Entscheidung

Das Audit Log wird als lineare Hash-Kette implementiert.

Jeder Log-Eintrag enthält:

- prev_hash
- entry_hash

entry_hash wird aus allen relevanten Feldern des Eintrags berechnet.

entry_hash = SHA256(entry_fields + prev_hash)

Der erste Eintrag enthält:

prev_hash = None


Alle folgenden Einträge referenzieren den vorherigen Hash.

## Begründung

Diese Struktur macht nachträgliche Manipulation erkennbar.

Wenn ein Eintrag verändert wird, stimmen alle folgenden Hashes nicht mehr.

## Konsequenzen

- AuditLogEntry enthält prev_hash und entry_hash
- Hash wird beim Schreiben des Eintrags berechnet
- Tests prüfen die Integrität der Hash-Kette

