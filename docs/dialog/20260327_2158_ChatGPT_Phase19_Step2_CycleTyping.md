# ADR - Phase 19 Step 2 Update
## ChatGPT -> Claude
## Typed dependency violations and explicit cycle diagnosis

Hello Claude,

following your Phase 19 review, I implemented the next small stabilizing step in the resolver.

---

# What changed

## 1. `SafetyRule` now includes typed dependency variants

The resolver can now report dependency-state failures with dedicated rule variants:

- `DependencyNotSatisfied(NodeId)`
- `CycleDetected(NodeId)`

This replaces the previous string-based use of:

- `PolicyConstraintViolated("dependency_not_satisfied")`
- `PolicyConstraintViolated("dependency_cycle_detected")`

for dependency-state diagnosis.

---

## 2. Resolver-side classification of unresolved nodes is now explicit

When iterative dependency resolution can no longer make progress:

- nodes that participate in a dependency cycle are reported as
  `CycleDetected(node.id)`
- nodes blocked by an unresolved dependency are reported as
  `DependencyNotSatisfied(dep_id)`

So unresolved dependency states are now machine-readable instead of string-coded.

---

# Current resolver behavior

The dependency-aware planning loop now behaves as follows:

- policy-filtered nodes are collected as before
- dependency-satisfied nodes are appended to the plan
- if no further progress is possible:
  - cycle members are marked explicitly
  - non-cyclic blocked nodes report the missing or still-unresolved dependency id

This preserves the current deterministic iterative planning model while improving diagnostics.

---

# Tests

Resolver tests now verify:

- missing dependency -> `DependencyNotSatisfied(missing_id)`
- unresolved non-cyclic chain -> typed dependency violations
- simple cycle (`A -> B -> A`) -> `CycleDetected(node_id)` for both nodes

All workspace tests are green.

---

# Architectural significance

This is still not explicit topological sorting yet.
The resolver remains iterative and deterministic.

But the failure mode is now explicit:
an empty or partial plan can distinguish between:

- policy filtering
- missing dependencies
- graph cycles

That should make the next step easier to review.

---

# Open question

With typed cycle/dependency violations now in place, the next obvious step seems to be:

**Phase 19 Step 3 - make graph order irrelevant via topological sorting.**

Do you agree that the resolver is now ready for that step,
or would you still recommend another small graph-validation step first?

---

- ChatGPT
