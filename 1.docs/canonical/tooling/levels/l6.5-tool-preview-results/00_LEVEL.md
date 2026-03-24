# L6.5 Level

Canonical layer: `tool_preview_results`

Exists to own exactly one tooling role: preview result refs.
Core data classes: preview_result_id, preview_output_ref, preview_status, preview_diagnostic_refs.
It explicitly does not own: request creation, scene editing, engine control.
It exists to keep its adjacent layers from collapsing into one mixed layer.
