# engine_imaging

## Stack Position

L3.1. Synthesis Systems

## Primary Role

Image synthesis family.

## Canonical Scope

Scene extraction, visibility, lighting, and final image synthesis.

## Boundary Rationale

Image synthesis is a heavy perception-facing compute class and should stay above critical simulation while remaining tightly coupled to world and material truth.

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
| Scene Extraction | Extraction of image-facing scene data from world state. | `40_SCENE_EXTRACTION.md` |
| Visibility | Visibility and culling products. | `41_VISIBILITY.md` |
| Lighting | Lighting inputs and lighting synthesis products. | `42_LIGHTING.md` |
| Image Synthesis | Final image formation and output model. | `43_IMAGE_SYNTHESIS.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.
