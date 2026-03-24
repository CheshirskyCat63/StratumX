# link_ingress_controls

## Role class
boundary layer

## Canonical role
publishes or stores only control boundary data

## Owned data kind
control boundary only

## Operational meaning
typed control envelopes for write-side control submission into public L4 control surfaces only

## Non-authority
does not own packets, does not own legality, does not own retries, does not own execution state

## One data kind law
This layer owns exactly `control boundary` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: L6 request publishers only

Below: compat facts, transport policies, legality gates, runtime handles, public L4 control surfaces

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
