# Library Baseline

## Global allowed baseline
- `serde` for typed serialization where persistence or interchange is required;
- `thiserror` for typed error surfaces;
- `smallvec` for compact lists;
- `indexmap` for stable keyed order;
- `crossbeam` for channels and worker coordination;
- `tracing` for diagnostics and task instrumentation;
- `parking_lot` only where local synchronization is justified;
- `camino` only in layers that carry filesystem-like refs;
- UI framework adapters only in family-specific outer adapters, never in conveyor-core layers.

## Baseline law
A library may be:
- required by a layer;
- allowed but unnecessary;
- conditionally allowed for outer adapters only;
- forbidden in the layer.

`10_LIBRARIES.md` for each layer must state which category applies and why.

## Explicitly forbidden in core lanes
- render backends;
- heavy GUI frameworks;
- model-provider SDKs;
- engine-private crates;
- build-system private runtimes.
