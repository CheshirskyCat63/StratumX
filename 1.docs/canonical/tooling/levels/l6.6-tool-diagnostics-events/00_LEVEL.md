# L6.6 Level

Canonical layer: `tool_diagnostics_events`

Exists to own exactly one tooling role: diagnostics observation envelopes.
Core data classes: lifecycle_event_ref, runtime_state_event_ref, diagnostics_event_ref, metrics_snapshot_ref.
It explicitly does not own: rendering, control submission, session creation, task planning.
It exists to keep its adjacent layers from collapsing into one mixed layer.
