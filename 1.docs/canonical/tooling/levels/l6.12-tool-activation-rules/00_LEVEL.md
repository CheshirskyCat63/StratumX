# L6.12 Level

Canonical layer: `tool_activation_rules`

Exists to own exactly one tooling role: activation rules.
Core data classes: tool_id, activation_trigger, required_capabilities, required_session_state, required_selection_kind.
It explicitly does not own: activation state, task execution, session ownership.
It exists to keep its adjacent layers from collapsing into one mixed layer.
