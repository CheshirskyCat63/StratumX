# Implementation Handoff

Implementation should preserve five physical crate zones:
- `sdk_bridge_types`
- `sdk_bridge_ingress`
- `sdk_bridge_egress`
- `sdk_bridge_compat`
- `sdk_bridge_refs`

Physical ownership:
- `sdk_bridge_types` owns fixed-width bridge ids, enums, bitflags, headers, and payload refs only
- `sdk_bridge_ingress` owns packet/control ingress lanes and ordered publication only
- `sdk_bridge_egress` owns immutable observation/metric batches, batch cursors, and read fanout only
- `sdk_bridge_compat` owns immutable fact snapshots, compiled verdict tables, transport policy tables, and legality lookup tables only
- `sdk_bridge_refs` owns immutable handle/ref/artifact-ref snapshot registries only

Rationale:
- hot write paths stay separate from hot read paths
- compat/gate logic stays pure and testable
- opaque exports stay pointer-free and easy to audit
- snapshot truth stays separate from stream truth
