# L6.F5 Level

Canonical layer: `physics_destruction_family`

Exists to own exactly one tooling role: physics and destruction family composition.
Core data classes: fracture lane, cluster lane, break-rule lane, debris lane, impact-response lane.
It explicitly does not own: engine simulation law, runtime scheduler, world truth.
It exists to keep its adjacent layers from collapsing into one mixed layer.
