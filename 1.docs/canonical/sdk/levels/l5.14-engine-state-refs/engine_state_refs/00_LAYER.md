# engine_state_refs

## Role class
handle/ref layer

## Canonical role
publishes or stores only state ref data

## Owned data kind
state ref only

## Operational meaning
engine state refs only

## Non-authority
does not own state payload bodies, does not own observations, does not own projections, does not own mutation

## One data kind law
This layer owns exactly `state ref` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: L6 readers only

Below: public L4 state export surfaces and observation anchors only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
