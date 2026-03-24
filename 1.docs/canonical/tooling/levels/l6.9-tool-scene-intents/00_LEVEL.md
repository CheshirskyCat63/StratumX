# L6.9 Level

Canonical layer: `tool_scene_intents`

Exists to own exactly one tooling role: scene workflow intents.
Core data classes: PlaceObjectIntent, MoveObjectIntent, RotateObjectIntent, ScaleObjectIntent, SceneGroupIntent, SceneDeleteIntent.
It explicitly does not own: world truth, runtime authority, selection truth.
It exists to keep its adjacent layers from collapsing into one mixed layer.
