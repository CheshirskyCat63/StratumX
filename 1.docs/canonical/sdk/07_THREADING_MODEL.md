# Threading Model

## Canonical threading classes
- ordered single-writer boundary publication
- immutable fanout boundary publication
- low-churn single-writer fact publication
- parallel evaluation with singular immutable verdict publication
- immutable fanout handle/ref publication

## Per-layer threading summary
- ordered single-writer: `link_ingress_packets`, `link_ingress_controls`
- immutable fanout: `link_egress_observations`, `link_egress_metrics`, all handle/ref layers
- low-churn single-writer: `compat_versions`, `compat_capabilities`, `compat_profiles`, `transport_policies`
- parallel evaluation then singular publish: `compat_verdicts`, `legality_gates`

## Concurrency law
- no layer owns a hidden scheduler.
- no layer owns mutable mega-state spanning multiple semantic classes.
- write-side order is preserved per engine session and per named boundary stream.
- read-side fanout is immutable and never coupled to UI refresh cycles.
