# StratumX SDK L5 Dependency Constitution

Dependencies are directional, typed, and role-constrained.

- only public L4 surfaces are visible below L5
- no L5 layer may depend on L6
- each local layer doc freezes its exact allowed imports
- convenience imports across semantic classes are forbidden
