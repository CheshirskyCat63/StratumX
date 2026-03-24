# link_ingress_packets

## Role class
boundary layer

## Canonical role
publishes or stores only packet boundary data

## Owned data kind
packet boundary only

## Operational meaning
typed packet envelopes for write-side submission into public L4 packet surfaces only

## Non-authority
does not own controls, does not own legality, does not own retries, does not own scheduling

## One data kind law
This layer owns exactly `packet boundary` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: L6 request publishers only

Below: compat facts, transport policies, legality gates, runtime handles, public L4 packet surfaces

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
