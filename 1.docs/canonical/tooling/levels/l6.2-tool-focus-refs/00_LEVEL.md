# L6.2 Level

Canonical layer: `tool_focus_refs`

Exists to own exactly one tooling role: focus refs.
Core data classes: focused_panel_ref, focused_view_ref, focused_object_ref, focused_mode_ref.
It explicitly does not own: selection truth, panel layout, rendering, activation rules.
It exists to keep its adjacent layers from collapsing into one mixed layer.
