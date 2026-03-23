# Fluid Field

## Role

Fluid field solve.

## Canonical Definition

`Fluid Field` is a canonical element of `engine_field` inside `L2. Critical Simulation Families`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Distributed volume, flow, containment, and leakage field updates across world partitions.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_field`:

- `engine_core`
- `engine_world`
- `engine_material`
- `engine_world_spatial`
- `engine_world_region`

## Layer Links

- parent crate: `engine_field`
- level: `L2. Critical Simulation Families`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
