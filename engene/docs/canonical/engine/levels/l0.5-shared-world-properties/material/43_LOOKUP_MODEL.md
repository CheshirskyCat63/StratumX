# Lookup Model

## Role

Parallel-safe material lookup.

## Canonical Definition

`Lookup Model` is a canonical element of `engine_material` inside `L0.5. Shared World Property Substrate`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Read-heavy lookup model exposing immutable descriptors and coefficients to upper systems.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_material`:

- `engine_core`
- `engine_handle`
- `engine_world`

## Layer Links

- parent crate: `engine_material`
- level: `L0.5. Shared World Property Substrate`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
