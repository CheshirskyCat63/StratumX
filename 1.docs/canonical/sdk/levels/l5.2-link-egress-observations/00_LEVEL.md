# l5.2-link-egress-observations

## Canonical layer
- link_egress_observations

## Role class
- boundary layer

## Why this layer exists
isolates runtime observations from metrics, projections, and tooling interpretation

## What this layer owns
typed read-side observation envelopes emitted by public L4 observation surfaces only

## What this layer does not do
does not own metrics, does not own projections, does not own interpretation, does not own fanout consumers

## Adjacent above
L6 read-model consumers only

## Adjacent below
public L4 observation surfaces only

## Collapse guard
This layer exists because observation boundary must remain separate from adjacent authorities.
