# l5.1-link-ingress-controls

## Canonical layer
- link_ingress_controls

## Role class
- boundary layer

## Why this layer exists
isolates runtime control envelopes from packets, gates, and engine execution semantics

## What this layer owns
typed control envelopes for write-side control submission into public L4 control surfaces only

## What this layer does not do
does not own packets, does not own legality, does not own retries, does not own execution state

## Adjacent above
L6 request publishers only

## Adjacent below
compat facts, transport policies, legality gates, runtime handles, public L4 control surfaces

## Collapse guard
This layer exists because control boundary must remain separate from adjacent authorities.
