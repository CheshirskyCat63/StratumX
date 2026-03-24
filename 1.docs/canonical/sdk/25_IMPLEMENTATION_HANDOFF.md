# Implementation Handoff

## Recommended implementation split
A conforming codebase should prefer four thin implementation zones:
- `sdk_bridge_boundary` for `link_ingress_*` and `link_egress_*`
- `sdk_bridge_compat` for `compat_*`, `transport_policies`, and `legality_gates`
- `sdk_bridge_runtime_refs` for `engine_*_handles` and `engine_*_refs`
- `sdk_bridge_types` for shared ids, enums, flags, statuses, and envelope types

## Mandatory implementation laws
- no zone may absorb L6 UI or workflow state
- no zone may import engine internals below public L4 surfaces
- no boundary zone may own retries or scheduler state
- no runtime-ref zone may deserialize hidden engine payloads

## Handoff rule
This package is safe to hand to developers only as a bridge-layer canon.
Any attempt to use it as editor-platform canon is misuse.
