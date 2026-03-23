# Spatial Addressing

## Role

Addressing in world space.

## Canonical Definition

`Spatial Addressing` is a canonical element of `engine_world_spatial` inside `L-0.1. World Spatial`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Cell, region, and spatial reference addressing structures over world space.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_world_spatial`:

- `engine_core`
- `engine_identity`
- `engine_handle`

## Layer Links

- parent crate: `engine_world_spatial`
- level: `L-0.1. World Spatial`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
