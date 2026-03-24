# L6.4 Level

Canonical layer: `tool_preview_requests`

Exists to own exactly one tooling role: preview request descriptors.
Core data classes: preview_request_id, preview_kind, source_scope, preview_policy_ref, refresh_mode.
It explicitly does not own: result ownership, runtime authority, diagnostics ownership.
It exists to keep its adjacent layers from collapsing into one mixed layer.
