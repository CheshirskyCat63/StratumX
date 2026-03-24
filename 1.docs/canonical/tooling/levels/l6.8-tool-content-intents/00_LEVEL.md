# L6.8 Level

Canonical layer: `tool_content_intents`

Exists to own exactly one tooling role: content workflow intents.
Core data classes: ImportContentIntent, RebindContentIntent, TagContentIntent, GroupContentIntent, ChunkAssignIntent.
It explicitly does not own: content truth, execution, preview ownership, import runtime.
It exists to keep its adjacent layers from collapsing into one mixed layer.
