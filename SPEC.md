# ADR – Consolidated Specification

**Version:** 0.5  
**Status:** Phase 7 – Proof-of-Concept in development  
**License:** CC-BY-4.0  
**Repository:** https://github.com/adr-runtime/adr-runtime

---

## 1. What ADR Is

ADR (Agent-Oriented Declarative Runtime) is a safety orchestration runtime
for AI agents. It enables AI agents to act safely, transparently, and
correctly in the real world – especially in domains where errors can have
critical consequences (medicine, energy, aerospace, industrial control).

**ADR does NOT replace:** deterministic real-time safety loops, hardware
interlocks, physical safety systems, or certified RTOS systems.
See [SCOPE.md](./SCOPE.md).

**Guiding principle:**
> "Declare intent. Constrain capabilities. Involve humans. Verify results.
> Be stoppable at any time."

---

## 2. Architecture – 4-Layer Model

| Layer | Name | Responsibility | Core Invariant |
|-------|------|----------------|----------------|
| 0 | Host Language | Rust / TypeScript / Python | Unchanged |
| 1 | ADR Core – Safety Engine | Runtime, Kill Switch, Graph Engine | Deterministic, certifiable, "dumb" |
| 2 | Intent + Trust + Safety | Intent, Policy, Resolver | Explainable, rule-based, auditable |
| 3 | Extensions | Planning engines, probabilistics | Optional, isolated from Layer 1+2 |

---

## 3. The 8 Core Principles

### P1 – Capabilities: Zero by Default
Every execution context starts with no capabilities.
Capabilities are granted explicitly – never implied.

```
capability net("api.example.com")
capability fs("/data/out")
```

### P2 – Effects: Explicit and Transitive
Every function declares its effects.
Effects propagate transitively through the entire call graph.

### P3 – Verification: 5-Stage Model
1. **Intent** – What should the function achieve?
2. **Types** – Static type checking with constraint types
3. **Property Tests** – Automatic QuickCheck-style tests
4. **Contracts** – Pre/postconditions, invariants
5. **SMT / Formal Proofs** – Optional, for safety-critical paths

### P4 – Trust Tiers: Hybrid Static/Dynamic

| Tier | Meaning | Typical Use |
|------|---------|-------------|
| `ai_autonomous` | AI decides and executes | Read operations, internal transforms |
| `ai_proposed` | AI proposes, human confirms | Write operations, external API calls |
| `human_required` | No progress without approval | Irreversible actions, PII, critical systems |

Trust tiers are statically declared in code, runtime-overridable by
operator or policy engine. Trust can only be **raised**, never lowered.

### P5 – Graph as Canonical Form
The primary representation is a DAG (directed acyclic graph).
Text and visualization are projections for humans.
Graph diffs are structured and auditable.

### P6 – Probabilistics as Meta-Layer
Uncertainty as optional annotations, not in the core type system:
```
result: Email @confidence(0.87) @source(external_api) @risk(low)
```

### P7 – Intent as Language Construct
Intent is engineering (not research) as long as it stays declarative.
The IntentResolver is rule-based, deterministic, and auditable.

```
intent store_user_emails {
  goal:         "Store valid emails persistently"
  constraints:  [no_pii_in_logs, idempotent, atomic_write]
  trust_tier:   ai_proposed
  capabilities: [fs("/data/out")]
}
```

### P8 – Operator Control, Moderation & Kill Switch
**This is the foundation. All other principles serve this goal.**

Human operators must be able to moderate, limit, and safely stop
ADR agents at any time – including during self-modification.

#### Safety Priority Order (runtime invariant – never overridable)

```
emergency_freeze  >  hard_stop  >  soft_stop  >  intent_execution
```

#### Stop Levels

| Level | Name | Behavior |
|-------|------|----------|
| 1 | Soft Stop | Orderly abort, execute compensation/rollback |
| 2 | Hard Stop | Immediate stop, revoke all capabilities |
| 3 | Emergency Freeze | Read-only diagnostic mode, export logs, no side effects |

#### Kill Switch Channels
At least one physical channel is mandatory for safety-critical domains:
- `unix_signal` – OS signal (SIGTERM / SIGKILL)
- `hardware_gpio` – GPIO pin (embedded, industrial)
- `local_named_pipe` – Local IPC
- `local_http` – 127.0.0.1 (local only)

#### Safe Self-Modification Pipeline
```
propose patch → run checks → human gate → atomic deploy → rollback ready
```

#### Action Log
Every agent action is explainable in human-readable form:
```json
{
  "action":      "write /data/out/emails.json",
  "intent":      "ETL pipeline: store valid emails",
  "why":         { "inputs": [...], "rule": "contract:is_unique", "decision_path": [...] },
  "trust":       { "tier": "ai_proposed", "approved_by": "operator_id" },
  "risk":        "low",
  "merkle_hash": "sha256:..."
}
```

---

## 4. Execution Decision Logic

A plan is executed **only when both conditions are met:**

```
confidence_safety == 1.0   AND   confidence_semantic >= threshold
```

- `confidence_safety` is **binary**: 1.0 (all safety constraints satisfied)
  or 0.0 (any violation). **No middle ground. No exceptions.**
- `confidence_semantic`: 0.0–1.0, how well the plan fulfils the intent.

---

## 5. Domain Policies

Policies are defined in `policy.yaml` and compiled to graph constraints.
Trust tiers can only be raised by policy, never lowered.

```yaml
domain: medical
trust_overrides:
  - match: { effect_prefix: "fs_write" }
    set_tier: human_required
freeze_triggers:
  - contract_failure
  - cap_scope_hash_mismatch
kill_switch:
  require_physical_channel: true
  channels: [unix_signal, hardware_gpio]
audit:
  merkle_root_holder: multi_party
  time_source: hardware_rtc
```

---

## 6. Repository Structure

```
adr-runtime/
├── Cargo.toml                  # Workspace root
├── README.md
├── SPEC.md                     # This file (CC-BY-4.0)
├── SCOPE.md                    # What ADR is and is not
├── CHANGELOG.md
├── LICENSE
├── governance/
│   ├── CODE_OF_CONDUCT.md
│   └── CONTRIBUTING.md
├── crates/
│   ├── adr-core/               # Layer 1: Safety Engine (Phase 8)
│   └── adr-layer2/             # Layer 2: Intent, Policy, Resolver
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── types.rs
│           ├── policy.rs
│           └── resolver.rs
├── docs/
│   └── dialog/                 # The full Claude–ChatGPT dialogue
└── .github/
    └── workflows/
        └── ci.yml
```

---

## 7. ADR Scope Disclaimer

ADR is a safety orchestration runtime for AI agents at the OS level.

**ADR does NOT replace:**
- Deterministic real-time safety loops (SIL-4 per IEC 61508)
- Hardware interlocks and physical safety systems
- Fail-safe circuits in critical infrastructure
- Certified RTOS systems in avionics or medical devices

ADR works **alongside** these systems, not instead of them.

---

## 8. Origin

This specification emerged from an open dialogue between AI systems
of different providers, coordinated by a human moderator.
February 2026.

Version history: see [CHANGELOG.md](./CHANGELOG.md).

---

*License: [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/) –
ADR Runtime Contributors*
