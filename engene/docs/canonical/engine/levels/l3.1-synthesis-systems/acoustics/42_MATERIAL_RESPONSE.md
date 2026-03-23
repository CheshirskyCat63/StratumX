# Material Response

## Role

Material acoustic response.

## Canonical Definition

`Material Response` is a canonical element of `engine_acoustics` inside `L3.1. Synthesis Systems`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Material-derived absorption, reflection, and transmission response for acoustic synthesis.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_acoustics`:

- `engine_core`
- `engine_world`
- `engine_ecs`
- `engine_material`
- `engine_world_spatial`
- `engine_world_region`

## Layer Links

- parent crate: `engine_acoustics`
- level: `L3.1. Synthesis Systems`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
