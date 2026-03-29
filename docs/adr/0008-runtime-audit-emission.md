# ADR 0008: Runtime Audit Emission

## Status
Proposed (Phase 24)

## Context

ADR now has:

- runtime-side approval validation before checkpoint nodes
- structured approval identities
- audit evidence fields that can carry approval identity
- a hash-chained audit entry type

What ADR still does not define is how runtime execution emits audit entries.

`AdrRuntime.execute_plan()` currently validates and executes nodes, but does not produce `ActionLogEntry` values itself.
That leaves a structural gap:

- approval-aware execution can happen
- but approval-aware execution is not yet reflected by runtime-emitted audit evidence

Before code is added, ADR should define what evidence the runtime is responsible for receiving and emitting.

## Decision

Runtime audit emission will be introduced via **explicit evidence inputs** rather than by making `AdrRuntime` reconstruct planning metadata on its own.

The runtime should receive the additional evidence it cannot derive locally, especially:

- policy version
- contract hash

The runtime may derive locally:

- graph version
- executed node id
- success/failure
- approval identity for the checkpoint currently being validated

In other words:

- Layer 2 / caller supplies planning-time evidence
- Layer 1 runtime supplies execution-time facts
- audit emission combines both into `ActionLogEntry`

## Rationale

This keeps layer boundaries explicit.

Reasons:

- `AdrRuntime` should not guess or reconstruct resolver-only metadata
- planning artifacts such as policy version and contract hash are not owned by the executor
- approval identity for a checkpoint is already present at runtime input time
- explicit inputs keep audit semantics deterministic and testable

## Consequences

Phase 24 should introduce a runtime-facing evidence input contract, for example:

```rust
pub struct ExecutionEvidence {
    pub policy_version: String,
    pub contract_hash:  String,
}
```

and execution APIs may evolve toward:

```rust
execute_plan(plan, graph, approvals, evidence)
```

Runtime-emitted `ActionLogEntry` values must include:

- graph version
- policy version
- contract hash
- approval identity when present

Open follow-up questions:

- whether runtime returns audit entries directly or writes them into an internal log
- whether failed approval checks should themselves emit audit entries
- how timestamps are sourced deterministically enough for the current phase

## Alternatives Considered

### Runtime reconstructs missing evidence on its own

Rejected because:

- policy version and contract hash are not runtime-owned facts
- it would blur the resolver/runtime boundary
- it would encourage hidden coupling between layers

### Keep runtime audit-less for now

Rejected because:

- approval-aware execution without runtime-emitted audit evidence leaves a visible integrity gap
- checkpoint execution should eventually be visible in the same hash chain as other execution facts
