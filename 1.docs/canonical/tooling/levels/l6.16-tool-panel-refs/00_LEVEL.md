# L6.16 Level

Canonical layer: `tool_panel_refs`

Exists to own exactly one tooling role: panel refs.
Core data classes: panel_id, panel_kind, panel_role, panel_binding_ref.
It explicitly does not own: layout ownership, rendering, workflow logic.
It exists to keep its adjacent layers from collapsing into one mixed layer.
