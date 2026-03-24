# l5.0-link-ingress-packets

## Canonical layer
- link_ingress_packets

## Role class
- boundary layer

## Why this layer exists
isolates runtime-bound packet envelopes from controls, compatibility facts, and engine execution

## What this layer owns
typed packet envelopes for write-side submission into public L4 packet surfaces only

## What this layer does not do
does not own controls, does not own legality, does not own retries, does not own scheduling

## Adjacent above
L6 request publishers only

## Adjacent below
compat facts, transport policies, legality gates, runtime handles, public L4 packet surfaces

## Collapse guard
This layer exists because packet boundary must remain separate from adjacent authorities.
