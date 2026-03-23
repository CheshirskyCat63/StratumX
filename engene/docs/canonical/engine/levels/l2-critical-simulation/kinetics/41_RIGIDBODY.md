# Rigidbody

## Role

Rigid-body solve.

## Canonical Definition

`Rigidbody` is a canonical element of `engine_kinetics` inside `L2. Critical Simulation Families`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Body state, mass/inertia, impulse integration, and constraint resolution for local motion.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_kinetics`:

- `engine_core`
- `engine_world`
- `engine_material`
- `engine_world_spatial`
- `engine_world_region`

## Layer Links

- parent crate: `engine_kinetics`
- level: `L2. Critical Simulation Families`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
