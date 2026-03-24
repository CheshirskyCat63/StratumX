# link_egress_observations

## Role class
boundary layer

## Canonical role
publishes or stores only observation boundary data

## Owned data kind
observation boundary only

## Operational meaning
typed read-side observation envelopes emitted by public L4 observation surfaces only

## Non-authority
does not own metrics, does not own projections, does not own interpretation, does not own fanout consumers

## One data kind law
This layer owns exactly `observation boundary` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: L6 read-model consumers only

Below: public L4 observation surfaces only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
