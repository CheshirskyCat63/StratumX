# Boundary Authority

## Canonical boundary owners
- `link_ingress_packets` owns packet-envelope submission to public L4 packet surfaces.
- `link_ingress_controls` owns control-envelope submission to public L4 control surfaces.
- `link_egress_observations` owns observation-envelope publication from public L4 observation surfaces.
- `link_egress_metrics` owns metric-envelope publication from public L4 metric surfaces.

## Authority restrictions
- no other L5 layer may submit into public L4.
- no other L5 layer may consume public L4 observations or metrics directly unless the local contract explicitly names an exported handle or ref surface.
- L6 may publish requests toward L5 but never bypasses the boundary mesh.
