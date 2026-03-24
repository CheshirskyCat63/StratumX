# link_egress_metrics

## Role class
boundary layer

## Canonical role
publishes or stores only metric boundary data

## Owned data kind
metric boundary only

## Operational meaning
typed read-side metric envelopes emitted by public L4 metric surfaces only

## Non-authority
does not own observations, does not own dashboards, does not own aggregation, does not own budgets

## One data kind law
This layer owns exactly `metric boundary` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: L6 read-model consumers only

Below: public L4 metric surfaces only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
