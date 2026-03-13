# ADR 0004: Capability Representation

## Status
Accepted (Phase 16)

## Context

ADR currently uses two representations of capabilities.

Layer 1 (runtime core):

CapabilitySet is implemented as a bitmask (`u64`).
This enables constant-time capability checks and deterministic behavior.

Layer 2 (resolver / planning):

Capabilities are represented as `Capability(String)`.

Example:

Capability("fs_write")
Capability("net_external")

This representation is easier for higher-level reasoning and policy logic.

## Decision

ADR uses a **dual capability representation**:

Layer 2 → symbolic capability names  
Layer 1 → numeric capability bitmasks

A mapping layer will translate between the two.

Example:

fs_write → CAP_FS_WRITE (bitmask)
net_external → CAP_NET_EXTERNAL

This mapping will be introduced in a later phase.

## Consequences

Advantages:

- Layer 1 remains deterministic and efficient
- Layer 2 remains expressive and policy-friendly

Trade-off:

A mapping table is required between symbolic and numeric capabilities.

This design keeps the runtime core minimal while allowing flexible policy logic.
