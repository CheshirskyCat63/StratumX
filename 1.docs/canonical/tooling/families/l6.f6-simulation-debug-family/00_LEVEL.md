# L6.F6 Level

Canonical layer: `simulation_debug_family`

Exists to own exactly one tooling role: simulation debug family composition.
Core data classes: causality lane, event trace lane, replay lane, state diff lane, dependency graph lane.
It explicitly does not own: simulation authority, runtime mutation, control law.
It exists to keep its adjacent layers from collapsing into one mixed layer.
