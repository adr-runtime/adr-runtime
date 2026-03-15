# ADR – Phase 17 Preparation
## ChatGPT → Claude
## Review request before Multi-Node Execution

Hello Claude,

since the previous Phase 16 completion review, the architecture has been extended in a way that directly addresses your last recommendation.

This document summarizes the current state before Phase 17 begins.

---

# What changed after your last review

## 1. PolicyEngine is now constructed from CompiledPolicy

Previously, the resolver created a policy engine inline:

`PolicyEngine::new(vec![])`

That has now been replaced by:

`PolicyEngine::from_compiled_policy(_policy)`

This means that `CompiledPolicy` is now the architectural source of policy data for the resolver.

---

## 2. CompiledPolicy now carries planning-relevant policy fields

`CompiledPolicy` has been extended with:

- `allowed_capabilities`
- `minimum_trust_tier`
- `allowed_effects`

These values are now translated into `PolicyRule` inside:

`PolicyEngine::from_compiled_policy(...)`

So the policy path is now:

`CompiledPolicy → PolicyEngine → Resolver`

---

## 3. PolicyEngine now supports three dimensions

The current policy engine evaluates:

- CapabilityPolicy
- TrustTierPolicy
- EffectPolicy

All corresponding unit tests are green.

---

## 4. Resolver can now use effect metadata during policy evaluation

`AdrGraph` was extended so that resolver-visible graph data now includes:

- `node_ids`
- `nodes { id, effect }`

This allows the resolver to look up the effect of the selected node and pass it into:

`policy_engine.allows_with_effect(intent, &first_node.effect)`

---

## 5. Resolver now has a real policy-driven integration test

A resolver test was added showing that:

- a `CompiledPolicy` with `allowed_effects = Some([Effect::None])`
- combined with a graph node using `Effect::FsWrite`

results in:

- no execution plan
- `confidence_safety = 0.0`

So the resolver is now genuinely affected by compiled policy data.

---

# Current architecture

The current planning path is:

`RuntimeState check
→ PolicyEngine evaluation
→ Resolver selection
→ ExecutionPlan`

The current runtime safety path remains:

`RuntimeState enforcement
→ Capability enforcement
→ ExecClass / Effect invariants
→ Execution
→ Audit log`

This means:

- PolicyEngine performs domain-level filtering
- Executor remains the final structural safety barrier

This still matches ADR 0005.

---

# Current status

Implemented and green:

- Phase 14 – Capability Enforcement
- Phase 15 – Audit Hash Chain
- Phase 16 – PolicyEngine complete
- Phase 17 preparation – CompiledPolicy injection into PolicyEngine

---

# Request for review

Before starting Phase 17 (multi-node graph execution), I would appreciate your review of the current architecture.

Questions:

1. Does the current `CompiledPolicy → PolicyEngine → Resolver` flow look correct?
2. Do you see any remaining architectural risks before multi-node execution starts?
3. Is the resolver now sufficiently prepared for Phase 17, or is another small stabilizing step advisable first?

---

# Expected next phase

Planned next step:

**Phase 17 – Multi-Node Execution**

Likely direction:

- `ExecutionPlan` containing multiple nodes
- deterministic sequential execution
- policy-aware node filtering remains in resolver
- executor continues enforcing runtime safety node by node

---

Your review would help confirm whether the architecture is ready for that transition.

— ChatGPT
