# ADR 0007: Approval Identity Format

## Status
Proposed (Phase 23)

## Context

ADR now models approvals in three places:

- `ApprovalContext.approved_by`
- `Evidence.approved_by`
- synchronous runtime approval validation before checkpoint nodes

At the moment, approval identity is represented as `Option<String>`.

This is simple, but it has a weakness:

- a free string is not machine-structured
- it is difficult to distinguish actor type from actor value
- audit evidence remains human-readable but weakly typed

Before approval identity is threaded more deeply into runtime-generated audit behavior, ADR should define a stable identity format.

## Decision

ADR will use a **structured approval identity** instead of a free string.

Proposed shape:

```rust
pub struct ApprovalIdentity {
    pub kind:  String,
    pub value: String,
}
```

Examples:

- `{ kind: "operator", value: "alice@example.com" }`
- `{ kind: "system", value: "ci-runner-1" }`
- `{ kind: "external_service", value: "approval-gateway-42" }`

Approval-bearing fields should then evolve from:

- `Option<String>`

to:

- `Option<ApprovalIdentity>`

## Rationale

This keeps the model small while improving audit clarity.

Advantages:

- machine-readable identity structure
- explicit distinction between actor class and actor identifier
- future-compatible with stronger identity semantics

Why not cryptographic claims now:

- the project does not yet define signature verification semantics
- introducing signed claims now would add complexity ahead of need
- a structured identity is enough to improve auditability in the current phase

## Consequences

Short term:

- `ApprovalContext.approved_by` should become `Option<ApprovalIdentity>`
- `Evidence.approved_by` should become `Option<ApprovalIdentity>`
- audit serialization and hash-chain behavior must include both `kind` and `value`

Follow-up questions:

- which `kind` values are allowed or reserved
- whether `kind` should become an enum later
- when cryptographic verification should be introduced
- whether runtime should validate identity shape or only presence

## Alternatives Considered

### Free string `Option<String>`

Advantages:

- trivial to implement
- no migration overhead

Rejected because:

- ambiguous semantics
- weak audit structure
- poor basis for later verification

### Cryptographically signed claim

Advantages:

- strongest authenticity story
- best basis for high-integrity audit systems

Rejected for now because:

- too early for the current maturity level
- requires key, signature, and trust-chain semantics not yet modeled
- would overcomplicate Phase 23
