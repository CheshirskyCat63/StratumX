# L6.7 Level

Canonical layer: `tool_diagnostics_views`

Exists to own exactly one tooling role: diagnostics projections.
Core data classes: diagnostics_view_id, event_group_refs, warning_rows, error_rows, state_summary_rows.
It explicitly does not own: event ownership, polling, engine access.
It exists to keep its adjacent layers from collapsing into one mixed layer.
