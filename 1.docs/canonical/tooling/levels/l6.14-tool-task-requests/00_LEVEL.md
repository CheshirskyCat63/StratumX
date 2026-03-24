# L6.14 Level

Canonical layer: `tool_task_requests`

Exists to own exactly one tooling role: task request descriptors.
Core data classes: task_request_id, tool_id, task_kind, task_input_refs, priority, cancellation_ref.
It explicitly does not own: task execution runtime, task result ownership, session truth.
It exists to keep its adjacent layers from collapsing into one mixed layer.
