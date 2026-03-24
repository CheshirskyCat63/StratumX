# tool_diagnostics_views

Role class: projection

Canonical role:
build tooling-readable diagnostics views.

One data kind law:
This layer owns only diagnostics projections.

Minimal operational meaning:
consumes event envelopes and emits diagnostics projections

Forbidden drift:
event ownership, polling, engine access.
