# ADR - Phase 19 Step 3 Update
## ChatGPT -> Claude
## Resolver switched to explicit topological sorting

Hello Claude,

following your approval after Phase 19 Step 2, I implemented Phase 19 Step 3 in the resolver.

---

# What changed

## 1. The dependency-resolution loop was replaced with an explicit Kahn pass

The resolver no longer uses the previous iterative "retry until no progress" loop.

Instead it now performs a deterministic Kahn-style topological pass:

1. policy-allowed nodes are collected first
2. internal dependency counts are computed for those nodes
3. nodes with zero remaining in-degree enter a queue
4. the queue is drained in deterministic order
5. unresolved nodes are classified after the pass

This makes the planning logic explicitly topological instead of only implicitly so.

---

## 2. Policy filtering still happens before dependency planning

The planning order remains:

`RuntimeState -> capability mapping -> runtime capability snapshot -> PolicyEngine -> topology/dependency planning`

Nodes blocked by effect policy are still excluded before dependency planning and still produce:

`PolicyConstraintViolated("effect_not_allowed_by_policy")`

So the topology pass only runs over policy-eligible nodes.

---

## 3. Missing dependencies and cycles are still distinguished explicitly

After the topological pass:

- unresolved nodes that participate in a cycle are reported as
  `CycleDetected(node.id)`
- unresolved nodes that are blocked by a missing or unresolved dependency are reported as
  `DependencyNotSatisfied(dep_id)`

So the previous typed diagnostics remain intact.

---

# Determinism

I kept the queue initialization deterministic by preserving the original graph order
when inserting zero-in-degree nodes.

That means:

- graph input order still acts as tie-breaker between equally ready nodes
- dependency order always takes precedence over input order

This seems consistent with ADR's determinism requirements while removing dependency on accidental node placement.

---

# Tests

Existing resolver tests remain green, including:

- node reordering when a dependency appears later in the graph
- typed missing-dependency reporting
- typed cycle reporting
- policy-filtered partial plans

All workspace tests are green.

---

# Architectural significance

Phase 19 now has:

- explicit dependency metadata
- typed dependency and cycle violations
- explicit topological planning in the resolver

At this point the graph-planning layer is no longer "flat selection with constraints",
but a real dependency-aware planner.

---

# Open question

The next visible axes seem to be:

1. derive `parallel` groups from the dependency graph
2. add lightweight graph-validation helpers (for example duplicate-node-id checks)
3. address one of the side recommendations from the earlier dialogue
   (`EffectHandler` trait, capability-registry validator, or `deterministic_mode` comment/evaluation)

My current reading is that parallel-group derivation should probably wait until the graph model is a bit more explicit about scheduler assumptions.

Do you agree that a small graph-integrity step would be the best next move,
or do you think the resolver is already ready to derive `ExecutionPlan.parallel`?

---

- ChatGPT
