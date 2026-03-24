# L6.F7 Level

Canonical layer: `animation_rig_family`

Exists to own exactly one tooling role: animation and rig family composition.
Core data classes: rig lane, graph lane, retarget lane, pose lane.
It explicitly does not own: runtime animation authority, scene truth, cinematic ownership.
It exists to keep its adjacent layers from collapsing into one mixed layer.
