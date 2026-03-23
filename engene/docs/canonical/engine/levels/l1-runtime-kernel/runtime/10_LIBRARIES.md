# Libraries

## External Library Baseline

| Library | Role in `engine_runtime` |
|---|---|
| `tracing` | Execution observability. |
| `rayon` | Baseline CPU parallel orchestration. |
| `crossbeam` | Queues and coordination primitives. |
| `parking_lot` | Low-overhead synchronization where justified. |
| `smallvec` | Low-allocation descriptors and batched runtime metadata. |

## Fit

Each listed dependency serves the primary role of `engine_runtime` and stays aligned to its pressure axis.
