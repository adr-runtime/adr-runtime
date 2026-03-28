# ADR 0006: Approval Lifecycle Model

## Status
Accepted (Phase 21)

## Context

ADR now has the Layer 2 data structures needed to express human approval requirements:

- `ExecutionPlan.checkpoints`
- `ResolverResult.open_human_gates`
- `ResolverResult.pending_approvals`
- per-node `trust_tier` in resolver-visible graph metadata

What ADR does not yet define is the execution-time approval lifecycle.

Before `AdrRuntime.execute_plan()` is changed, three questions must be answered:

1. how approvals are represented at execution time
2. whether execution pauses mid-plan or receives approvals up front
3. how rejection or timeout is modeled

Without a clear answer, introducing runtime-side checkpoint behavior would create an unstable API.

## Decision

ADR will use a **pre-approval model** as the default approval lifecycle.

That means:

- approvals are supplied to execution before the relevant plan is started
- `execute_plan()` remains deterministic and does not pause mid-run
- checkpoint validation is performed synchronously against the provided approval records

Resume-style execution handles are deferred unless later phases prove they are necessary.

## Rationale

The pre-approval model fits ADR's current architecture better than checkpoint-resume.

Reasons:

- ADR already prefers deterministic, prevalidated execution
- no persistent runtime state machine is required
- no partially executed suspended plan has to be resumed
- the API surface stays smaller and easier to audit

Checkpoint-resume remains possible in a future phase, but it should not be the default starting model.

## Consequences

Short term:

- Layer 2 can continue producing checkpoint and approval metadata
- Layer 1 should not yet add pause/resume semantics
- Phase 21 can introduce an approval input contract for execution
- Phase 21 introduces `execute_plan(plan, graph, approvals: &[ApprovalContext])`
- the runtime validates approvals synchronously before each checkpoint node

Planned follow-up questions:

- what exact identity format `approved_by` should use
- who is allowed to approve, and how that identity is verified
- how approval rejection is represented
- how approval expiry or timeout is represented
- whether executor input should accept all approvals or only checkpoint-relevant approvals

## Alternatives Considered

### Checkpoint-resume model

Example:

- `execute_plan()` runs until a checkpoint
- runtime returns an execution handle
- caller resumes later with approval data

Rejected for now because:

- it requires suspended execution state
- it expands runtime API complexity early
- it is harder to keep deterministic and audit-friendly

### Executor preflight without lifecycle definition

Example:

- runtime rejects plans containing pending approvals
- no approval contract exists yet

Rejected because:

- it introduces a runtime branch without telling the caller how to proceed
- `should_execute()` already provides the correct pre-execution human-review signal
- no additional runtime preflight is needed before the approval lifecycle contract exists
