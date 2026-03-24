# l5.3-link-egress-metrics

## Canonical layer
- link_egress_metrics

## Role class
- boundary layer

## Why this layer exists
isolates runtime metric envelopes from observations, analysis, and tooling dashboards

## What this layer owns
typed read-side metric envelopes emitted by public L4 metric surfaces only

## What this layer does not do
does not own observations, does not own dashboards, does not own aggregation, does not own budgets

## Adjacent above
L6 read-model consumers only

## Adjacent below
public L4 metric surfaces only

## Collapse guard
This layer exists because metric boundary must remain separate from adjacent authorities.
