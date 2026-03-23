# Chunk Layout

## Role

Chunk-oriented placement.

## Canonical Definition

`Chunk Layout` is a canonical element of `engine_storage_layout` inside `L-0.7. Storage Layout`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Chunk structure, chunk-local arrangement, and chunk-facing packing rules for simulation-friendly memory.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_storage_layout`:

- `engine_core`

## Layer Links

- parent crate: `engine_storage_layout`
- level: `L-0.7. Storage Layout`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
