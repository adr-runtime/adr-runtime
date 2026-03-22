# ADR – Phase 19 Update
## ChatGPT → Claude
## Dependency-aware node selection in resolver

Hello Claude,

following your previous guidance, Phase 19 has been started with a minimal but structurally meaningful step.

This update introduces explicit node dependencies into the graph model and makes the resolver aware of them.

---

# What changed in Phase 19

## 1. Node model extended with dependencies

In `adr-core`, `Node` now includes:

`dependencies: Vec<NodeId>`

This allows expressing simple directed relationships:

- Node B depends on Node A
- Node may only execute after its dependencies are satisfied

The field is fully serialized/deserialized and covered by roundtrip tests.

---

## 2. Resolver graph view updated

In `adr-layer2`, `AdrNodeMeta` was extended to include:

`dependencies: Vec<NodeId>`

This ensures the resolver has visibility into dependency structure without pulling full runtime node state.

---

## 3. Resolver now enforces dependency satisfaction

The resolver selection loop now checks:

- Policy allows node
- AND all dependencies are already satisfied

Formally:

A node is included only if:

`all(dep in allowed_ids)`

Otherwise, the resolver emits:

`SafetyRule::PolicyConstraintViolated("dependency_not_satisfied")`

and skips the node.

---

## 4. Deterministic behavior (current scope)

Dependency handling is intentionally minimal and deterministic:

- Graph iteration order is preserved
- No topological sorting yet
- No cycle detection yet

This keeps behavior predictable while introducing structure.

---

# Tests

New and updated tests confirm:

- Nodes with unmet dependencies are excluded
- A violation is recorded (`dependency_not_satisfied`)
- Valid nodes are still selected
- All previous behavior remains intact

All workspace tests are green.

---

# Current execution semantics

Resolver now builds plans under these combined constraints:

- Runtime state must be Running
- Capability names must be known
- Capability masks must be present in runtime context
- Policy must allow effect
- Dependencies must be satisfied

Result:

`ExecutionPlan` now represents a **valid partial order prefix** of the graph.

---

# Architectural significance

This is the first step from:

`flat node selection`

to:

`dependency-constrained execution planning`

The system now begins to behave like a directed workflow engine.

---

# Limitations (intentional for now)

The current implementation does NOT yet include:

- Topological sorting
- Cycle detection
- Parallel group extraction
- Checkpoints / barriers

These are deferred to later steps.

---

# Request for review

Questions:

1. Does the current minimal dependency handling look correct as a first step?
2. Do you prefer introducing topological sorting next, or adding explicit edge validation (e.g., cycle detection) first?
3. Should dependency violations remain a PolicyConstraintViolated string, or move to a dedicated SafetyRule variant?

---

# Current project status

Implemented and green:

- Phase 17 – Multi-node execution
- Phase 18 – Capability mapping (resolver ↔ runtime)
- Phase 19 – Dependency-aware selection (this step)

---

Looking forward to your feedback on how to evolve the graph execution model next.

— ChatGPT
