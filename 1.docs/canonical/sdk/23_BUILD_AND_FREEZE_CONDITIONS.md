# Build And Freeze Conditions

L5 is frozen only when:
- all bridge classes are typed and bounded
- artifact refs are distinct from state refs
- verdicts are distinct from facts
- controls are distinct from packets
- no direct upper-layer truth appears inside L5
- no hidden cache, graph, or disk store appears inside L5
- no hot-path publication requires per-envelope heap allocation by default
- ingress publication, snapshot publication, and egress batch publication are distinct physical planes
- legality and compatibility hot paths are resolved through declared compiled tables rather than ad-hoc mutable logic
- every declared level, including `l5.15-engine-artifact-refs`, has complete local layer contracts
