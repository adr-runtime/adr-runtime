# ADR Design Principles

These principles guide the architecture of ADR (Agent-Oriented Declarative Runtime).
They are intended to prevent architectural drift as the system evolves.

The system is designed for **safety-critical orchestration**, not for replacing
deterministic control systems.

---

# 1. Resolver Proposes, Executor Enforces

The resolver suggests execution plans.

The executor is the **final safety authority**.

The executor MUST always verify:

- RuntimeState
- Capability enforcement
- ExecClass constraints
- Effect restrictions

Even if the resolver makes a mistake, the executor must prevent unsafe execution.

---

# 2. Safety Before Policy

Safety checks must always occur before policy evaluation.

Order of responsibility:

Runtime safety  
→ Capability enforcement  
→ Policy filtering  
→ Execution planning

Policy must never bypass core runtime safety mechanisms.

---

# 3. Minimal Trusted Core

Layer 1 (runtime) must remain:

- deterministic
- small
- auditable
- independent from AI logic

AI reasoning and planning belong to Layer 2.

Layer 1 must remain safe even if Layer 2 behaves incorrectly.

---

# 4. Explicit Capability Boundaries

Capabilities represent permission to use system effects.

Examples:

- filesystem access
- network communication
- external actuator control

Capabilities must be:

- explicitly declared
- enforced in the executor
- logged in the audit system

No implicit capabilities are allowed.

---

# 5. Auditable Execution

All executed actions must produce an audit record.

The audit log must be:

- deterministic
- tamper-evident
- ordered

The current design uses a **linear hash chain** to detect modification.

Future extensions may include:

- Merkle trees
- external timestamp anchors
- distributed verification.

---

# 6. Small, Reviewable Steps

Architecture changes must be introduced in small increments.

Typical development cycle:

1. Minimal skeleton
2. Architecture review
3. Controlled extension

Large unreviewed changes are avoided.

---

# 7. Clear Layer Separation

ADR has two main layers:

Layer 1 — Runtime Core  
Layer 2 — Resolver / Policy / Planning

Layer 2 must not compromise the safety guarantees of Layer 1.

Layer 1 must remain valid even if Layer 2 fails.

---

# Summary

ADR prioritizes:

- safety
- determinism
- auditability
- minimal trusted runtime

This document acts as a guardrail for future architectural decisions.