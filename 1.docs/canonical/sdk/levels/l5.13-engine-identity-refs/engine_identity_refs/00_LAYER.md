# engine_identity_refs

## Role class
handle/ref layer

## Canonical role
publishes or stores only identity ref data

## Owned data kind
identity ref only

## Operational meaning
engine identity refs only

## Non-authority
does not own handles, does not own state, does not own editor-facing asset identities, does not own mutation

## One data kind law
This layer owns exactly `identity ref` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: L6 readers, object-handle readers, and state-ref readers only

Below: public L4 identity export surfaces only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
