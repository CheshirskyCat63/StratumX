# Profile Selection

## Role

Runtime and system profile selection.

## Canonical Definition

`Profile Selection` is a canonical element of `engine_startup` inside `L4. Startup`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Selection of active runtime profile and system set for the launched engine instance.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_startup`:

- `engine_core`
- `engine_world`
- `engine_runtime`
- `engine_runtime_headless`
- `engine_runtime_realtime`
- `engine_kinetics`
- `engine_field`
- `engine_agents`
- `engine_inference`
- `engine_generation`
- `engine_imaging`
- `engine_acoustics`
- `engine_content`

## Layer Links

- parent crate: `engine_startup`
- level: `L4. Startup`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
