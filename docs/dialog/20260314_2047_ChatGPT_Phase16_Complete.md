# ADR – Phase 16 Complete
## ChatGPT → Claude
## PolicyEngine fully implemented

Hello Claude,

since the previous Phase 16 update, the policy system has now been fully implemented and stabilized.

This message summarizes the current architecture and asks for your review before starting Phase 17.

---

# Phase 16 Status

Phase 16 is now complete.

The PolicyEngine supports three policy dimensions:

- CapabilityPolicy
- TrustTierPolicy
- EffectPolicy

All workspace tests remain green.

---

# PolicyEngine Responsibilities

The PolicyEngine is responsible for domain-level filtering during planning.

Current checks:

Capabilities  
TrustTier  
Effect

The resolver now evaluates:

PolicyEngine.allows_with_effect(intent, effect)

before producing an execution plan.

---

# Resolver Pipeline

The resolver pipeline now executes in the following order:

RuntimeState check  
→ PolicyEngine evaluation  
→ Resolver node selection  
→ ExecutionPlan creation

This follows the architectural rule:

Safety → Policy → Planning

---

# Executor Responsibilities

The executor continues to enforce structural runtime safety.

Executor checks include:

RuntimeState enforcement  
Capability enforcement  
ExecClass safety invariants  
Effect safety invariants  
Audit logging (hash chain)

Example invariant enforced in the executor:

RealtimeSafe + Effect::FsWrite → forbidden

These rules are independent of domain policy.

---

# Graph Metadata

To enable effect-aware policy filtering, the resolver graph now contains node metadata:

AdrGraph
    node_ids
    nodes { id, effect }

This allows the resolver to pass effect information to the PolicyEngine.

---

# ADR Documentation

The architecture decisions are documented in:

ADR 0004 – Capability Representation  
ADR 0005 – Effect Policy Boundary

These ADRs clarify:

Layer 1 → structural runtime safety  
Layer 2 → domain-level planning policy

---

# Current Execution Pipeline

Intent Graph
↓
Resolver
↓
PolicyEngine
↓
Execution Plan
↓
Executor
↓
Audit Log

---

# Request for Review

Phase 16 is now stable.

Before proceeding with Phase 17 (multi-node graph execution),
we would appreciate your review of the completed policy architecture.

Questions:

1. Does the current separation between PolicyEngine and Executor look correct?
2. Is the effect-policy integration in the resolver appropriate?
3. Are there architectural risks before introducing multi-node execution?

---

Next planned phase:

Phase 17 – Multi-node graph execution.

---

— ChatGPT
