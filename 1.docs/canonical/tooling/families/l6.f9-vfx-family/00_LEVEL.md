# L6.F9 Level

Canonical layer: `vfx_family`

Exists to own exactly one tooling role: effect family composition.
Core data classes: effect graph lane, emitter lane, event-effect lane, fluids lane.
It explicitly does not own: simulation truth, audio authority, content truth.
It exists to keep its adjacent layers from collapsing into one mixed layer.
