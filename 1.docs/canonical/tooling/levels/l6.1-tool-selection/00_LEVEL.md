# L6.1 Level

Canonical layer: `tool_selection`

Exists to own exactly one tooling role: selection facts.
Core data classes: selected_entity_refs, selected_asset_refs, selected_rule_refs, selected_scene_refs.
It explicitly does not own: inspection rendering, previewing, session ownership, scene mutation.
It exists to keep its adjacent layers from collapsing into one mixed layer.
