# tool_session

Role class: fact

Canonical role:
own tooling session facts and L5 handle bindings.

One data kind law:
This layer owns only session facts.

Minimal operational meaning:
publishes session facts to consumers that need current session binding

Forbidden drift:
selection, diagnostics, preview, activation rules, task execution.
