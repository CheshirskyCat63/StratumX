# tool_inspection_views

Role class: projection

Canonical role:
build read-side property and schema projections.

One data kind law:
This layer owns only inspection projections.

Minimal operational meaning:
consumes facts and emits inspection views

Forbidden drift:
selection ownership, write authority, engine mutation.
