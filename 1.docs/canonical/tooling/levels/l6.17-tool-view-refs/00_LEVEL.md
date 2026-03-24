# L6.17 Level

Canonical layer: `tool_view_refs`

Exists to own exactly one tooling role: view refs.
Core data classes: view_id, view_kind, view_binding_ref, view_mode_flags.
It explicitly does not own: data ownership, rendering logic, workflow logic, panel layout.
It exists to keep its adjacent layers from collapsing into one mixed layer.
