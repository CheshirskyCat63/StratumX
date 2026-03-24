# L6.15 Level

Canonical layer: `tool_task_results`

Exists to own exactly one tooling role: task result refs.
Core data classes: task_result_id, source_task_request_id, result_status, result_refs, diagnostic_refs.
It explicitly does not own: task scheduling, rendering, session truth, retries.
It exists to keep its adjacent layers from collapsing into one mixed layer.
