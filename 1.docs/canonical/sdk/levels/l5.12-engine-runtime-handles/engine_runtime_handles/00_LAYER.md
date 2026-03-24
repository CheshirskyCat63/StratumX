# engine_runtime_handles

## Role class
handle/ref layer

## Canonical role
publishes or stores only runtime handle ref data

## Owned data kind
runtime handle ref only

## Operational meaning
runtime surface handles only

## Non-authority
does not own sessions, does not own observations, does not own metrics, does not own runtime scheduling

## One data kind law
This layer owns exactly `runtime handle ref` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: boundary, facts, gates, and L6 readers only

Below: public L4 runtime surface handles only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
