# L6.F2 Level

Canonical layer: `scene_family`

Exists to own exactly one tooling role: scene family composition.
Core data classes: placement lane, transform lane, grouping lane, hierarchy lane.
It explicitly does not own: world truth, runtime ownership, build/package logic.
It exists to keep its adjacent layers from collapsing into one mixed layer.
