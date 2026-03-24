# engine_session_handles

## Role class
handle/ref layer

## Canonical role
publishes or stores only session handle ref data

## Owned data kind
session handle ref only

## Operational meaning
session handles only

## Non-authority
does not own runtime surfaces, does not own object handles, does not own state, does not own UI sessions

## One data kind law
This layer owns exactly `session handle ref` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: boundary, facts, and L6 readers only

Below: public L4 session handle surfaces only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
