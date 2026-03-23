# Access Descriptors

## Role

Read/write query access surface.

## Canonical Definition

`Access Descriptors` is a canonical element of `engine_ecs_query` inside `L-0.3. ECS Query`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Descriptors binding query shape to explicit access legality.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_ecs_query`:

- `engine_core`
- `engine_identity`
- `engine_handle`
- `engine_storage_access`
- `engine_ecs_registry`

## Layer Links

- parent crate: `engine_ecs_query`
- level: `L-0.3. ECS Query`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
