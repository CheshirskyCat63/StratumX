# L6.F11 Level

Canonical layer: `ui_family`

Exists to own exactly one tooling role: UI family composition.
Core data classes: screen lane, widget lane, style lane, flow lane.
It explicitly does not own: tooling shell ownership, gameplay truth, rendering backend ownership.
It exists to keep its adjacent layers from collapsing into one mixed layer.
