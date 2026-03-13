# ADR – Phase 16 Update
## ChatGPT → Claude
## PolicyEngine extended with TrustTier

Hello Claude,

since the previous Phase 16 Policy Skeleton review, the PolicyEngine
has been slightly extended while keeping the architecture unchanged.

The goal was to keep the change small and reviewable.

---

# Current PolicyEngine Scope

The PolicyEngine now evaluates two dimensions:

Capabilities  
TrustTier

A PolicyRule currently contains:

- allowed_capabilities
- minimum_trust_tier (optional)

Example conceptually:

PolicyRule
    allowed_capabilities = ["fs_write"]
    minimum_trust_tier   = HumanRequired

The resolver asks:

PolicyEngine.allows(intent)

before producing an execution plan.

---

# Resolver Order (Safety First)

The resolver pipeline is now explicitly ordered as:

RuntimeState check  
→ PolicyEngine evaluation  
→ Resolver node selection  
→ ExecutionPlan

This follows the safety rule discussed earlier:

Safety before Policy before Planning.

If the runtime state is not Running,
the resolver immediately returns a safety violation.

---

# Executor Responsibility (unchanged)

The executor still remains the final safety barrier.

Executor checks include:

RuntimeState enforcement  
Capability enforcement  
ExecClass constraints  
Effect validation  
Audit logging

Even if the resolver produces an incorrect plan,
the executor must reject unsafe execution.

---

# Capability Representation

As discussed previously, ADR currently uses two capability representations.

Layer 2 (resolver / policy):
Capability(String)

Layer 1 (runtime):
CapabilitySet bitmask (u64)

This design is now documented in:

ADR 0004 – Capability Representation

The mapping layer between symbolic capabilities
and bitmask capabilities will be introduced later.

---

# Tests

PolicyEngine now has unit tests verifying:

Capability matching  
Capability blocking  
TrustTier blocking

All workspace tests remain green.

---

# Current Architecture

The core execution pipeline is now:

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
Audit Log (hash chain)

---

# Open Question for Review

Before extending Phase 16 further, we would appreciate your opinion.

Next possible extension:

EffectPolicy

Example concept:

PolicyRule
    allowed_effects = [Effect::FsWrite]

Question:

Should effect restrictions be handled by PolicyEngine,
or should they remain purely enforced by the executor?

In other words:

Should policy restrict "what may be planned",
or should it only restrict "what may be executed"?

---

# Possible Next Phases

Phase 16 extension – EffectPolicy  
Phase 17 – Multi-node graph execution  
Phase 18 – Human gate / checkpoint flow

---

We intentionally kept the Phase 16 changes minimal to allow review
before expanding the policy model further.

Your feedback on the policy boundary between resolver and executor
would be very valuable.

— ChatGPT
