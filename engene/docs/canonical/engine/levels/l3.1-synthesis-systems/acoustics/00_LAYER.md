# engine_acoustics

## Stack Position

L3.1. Synthesis Systems

## Primary Role

Acoustic synthesis family.

## Canonical Scope

Propagation, reflection and occlusion, material response, and acoustic output formation.

## Boundary Rationale

Acoustic synthesis is a heavy perception-facing compute class with its own spatial, material, and source-group pressure axis.

## Upward Consumers

- `engine_runtime`
- `engine_startup`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_world` — World truth boundary.
- `engine_ecs` — ECS substrate.
- `engine_material` — Shared world property substrate.
- `engine_world_spatial` — Spatial substrate.
- `engine_world_region` — Region substrate.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Propagation | Spatial sound propagation model. | `40_PROPAGATION.md` |
| Reflection and Occlusion | Reflection, transmission, and occlusion products. | `41_REFLECTION_AND_OCCLUSION.md` |
| Material Response | Acoustic response derived from material substrate. | `42_MATERIAL_RESPONSE.md` |
| Acoustic Outputs | Final acoustic output model. | `43_ACOUSTIC_OUTPUTS.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.
