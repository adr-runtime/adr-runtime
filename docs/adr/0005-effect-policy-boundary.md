# ADR 0005: Effect Policy Boundary

## Status
Accepted

## Context

ADR currently validates effects in two different places:

- Layer 1 (executor)
- Layer 2 (policy engine)

This can be confusing unless the responsibilities are clearly separated.

Example:

- `RealtimeSafe + Effect::FsWrite` must never be allowed
- a domain policy may additionally restrict effects, for example:
  only `Effect::None` allowed in a given environment

## Decision

Effect validation is split into two responsibilities:

### Executor
The executor enforces **structural safety invariants**.

Examples:

- `RealtimeSafe + Effect::FsWrite` → forbidden
- `RealtimeSafe + Effect::NetExternal` → forbidden

These rules are always enforced, independent of domain policy.

### PolicyEngine
The PolicyEngine applies **domain-level effect filters**.

Examples:

- only `Effect::None` allowed in a specific domain
- only `Effect::FsWrite` allowed for a given policy set

These rules are not structural safety invariants.
They are contextual restrictions used during planning.

## Rationale

This separation keeps the architecture clean:

- Layer 1 remains the final safety barrier
- Layer 2 remains responsible for planning-time filtering

The executor must never depend on policy correctness for core safety.

## Consequences

- `EffectPolicy` may be added to the PolicyEngine
- executor-side effect checks remain unchanged
- effect-related runtime safety tests must remain green
