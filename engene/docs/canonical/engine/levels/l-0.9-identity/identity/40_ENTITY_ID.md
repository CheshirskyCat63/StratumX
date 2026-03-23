# Entity Id

## Role

Primary entity identity.

## Canonical Definition

`Entity Id` is a canonical element of `engine_identity` inside `L-0.9. Identity Primitives`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Stable typed entity identifier with generation-aware semantics over entity lifetime.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_identity`:

- `engine_core`

## Layer Links

- parent crate: `engine_identity`
- level: `L-0.9. Identity Primitives`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
