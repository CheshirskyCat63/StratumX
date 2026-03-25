# L5 Internal Contract Model

## Physical planes
- immutable bridge snapshots
- ordered ingress lanes
- immutable egress batches and cursors

## Write contract
- reads immutable fact snapshots
- resolves verdicts and legality through compiled lookup tables
- applies transport policy without semantic widening
- publishes packets or controls through ordered ingress lanes

## Read contract
- receives typed L4 publications through public batch or snapshot surfaces
- normalizes them without semantic widening
- exposes fanout-safe immutable views upward as snapshots, batches, and cursors

## Internal law
No internal contract may require editor state, assistant state, campaign state, or planning state.
No internal contract may duplicate engine truth into a second mutable authority store.
