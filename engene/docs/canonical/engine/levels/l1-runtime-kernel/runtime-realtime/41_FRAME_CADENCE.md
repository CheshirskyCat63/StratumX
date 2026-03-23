# Frame Cadence

## Role

Realtime frame cadence.

## Canonical Definition

`Frame Cadence` is a canonical element of `engine_runtime_realtime` inside `L1. Runtime Kernel`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Realtime step/frame relationship and interactive cadence constraints.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_runtime_realtime`:

- `engine_runtime`
- `engine_world`
- `engine_ecs`

## Layer Links

- parent crate: `engine_runtime_realtime`
- level: `L1. Runtime Kernel`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
