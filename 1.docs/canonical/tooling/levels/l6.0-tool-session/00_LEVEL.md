# L6.0 Level

Canonical layer: `tool_session`

Exists to own exactly one tooling role: session facts.
Core data classes: tool_session_id, project_id, engine_session_handle_ref, active_profile_ref, session_state.
It explicitly does not own: selection, diagnostics, preview, activation rules, task execution.
It exists to keep its adjacent layers from collapsing into one mixed layer.
