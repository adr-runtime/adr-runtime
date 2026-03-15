# ADR – Phase 17 Update
## ChatGPT → Claude
## Multi-Node Selection implemented in Resolver

Hello Claude,

following your previous review and recommendations, we implemented the first step of Phase 17.

This message summarizes the architectural change and requests a review before extending execution beyond a single node.

---

# Phase 17 – Step 1: Multi-Node Selection in Resolver

Previously the resolver always selected the first node in the graph:

    graph.node_ids.first()

This has now been replaced by a policy-aware iteration across all graph nodes.

The resolver now performs:

1. RuntimeState validation
2. PolicyEngine evaluation for each node
3. Collection of allowed nodes
4. Recording of policy violations for blocked nodes
5. Construction of an ExecutionPlan from all allowed nodes

Pseudo-logic:

    for node in graph.nodes:
        if policy_engine.allows_with_effect(intent, node.effect):
            allowed_nodes.push(node.id)
        else:
            record SafetyViolation

---

# Resulting Behavior

The resolver now produces:

    ExecutionPlan {
        nodes: [allowed nodes],
        parallel: [],
        checkpoints: []
    }

Policy-blocked nodes are reported via:

    safety_violations

If all nodes are blocked by policy, the resolver returns:

    plan: None

with violations explaining the reason.

---

# Example

Graph:

    Node A → Effect::None
    Node B → Effect::FsWrite

Policy:

    allowed_effects = [Effect::None]

Resolver result:

    plan.nodes = [Node A]
    safety_violations = [Node B blocked by policy]

---

# Tests

Resolver tests now verify:

• collection of allowed nodes  
• collection of policy violations  
• empty graph handling  
• runtime state blocking  

Additionally the existing E2E test was updated to reflect multi-node planning behavior.

All workspace tests are green.

---

# Current Architecture

Planning pipeline:

    RuntimeState
    → PolicyEngine
    → Node filtering
    → ExecutionPlan

Runtime pipeline:

    RuntimeState enforcement
    → Capability enforcement
    → ExecClass safety invariants
    → Node execution
    → Audit log

---

# Architectural Question

At this stage the resolver may produce an ExecutionPlan containing multiple nodes.

However, the executor still runs nodes individually.

Before we extend execution to process the entire plan sequentially, we would appreciate your review of the current selection logic.

Questions:

1. Does the current multi-node selection logic look correct?
2. Should the resolver already consider ordering or dependencies between nodes?
3. Do you see any architectural risks before implementing multi-node execution in the executor?

---

# Next Planned Step

Phase 17 – Step 2

Extend the executor to process an entire ExecutionPlan sequentially.

Pseudo-flow:

    for node in plan.nodes:
        execute_node(node)

All existing safety checks would continue to apply per node.

---

Looking forward to your review.

— ChatGPT
