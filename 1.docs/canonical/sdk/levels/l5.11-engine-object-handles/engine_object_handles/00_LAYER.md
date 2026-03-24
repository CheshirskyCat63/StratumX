# engine_object_handles

## Role class
handle/ref layer

## Canonical role
publishes or stores only object handle ref data

## Owned data kind
object handle ref only

## Operational meaning
object handles only

## Non-authority
does not own sessions, does not own state, does not own object metadata, does not own editor objects

## One data kind law
This layer owns exactly `object handle ref` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: boundary, gates, and L6 readers only

Below: public L4 object handle surfaces only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
