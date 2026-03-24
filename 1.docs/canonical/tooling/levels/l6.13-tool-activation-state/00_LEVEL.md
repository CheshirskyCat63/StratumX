# L6.13 Level

Canonical layer: `tool_activation_state`

Exists to own exactly one tooling role: activation facts.
Core data classes: tool_id, activation_state, last_activated_at, activation_reason_ref, active_task_refs.
It explicitly does not own: rule definition, task planning, session ownership.
It exists to keep its adjacent layers from collapsing into one mixed layer.
