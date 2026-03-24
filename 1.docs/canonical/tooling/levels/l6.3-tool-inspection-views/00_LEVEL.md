# L6.3 Level

Canonical layer: `tool_inspection_views`

Exists to own exactly one tooling role: inspection projections.
Core data classes: inspection_view_id, source_ref, property_rows, schema_projection_ref, diagnostic_overlay_refs.
It explicitly does not own: selection ownership, write authority, engine mutation.
It exists to keep its adjacent layers from collapsing into one mixed layer.
