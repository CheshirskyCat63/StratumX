# engine_content

## Stack Position

L3.2. Resource Systems

## Primary Role

Resource and content processing family.

## Canonical Scope

Ingest, transform, metadata, and resource products.

## Boundary Rationale

Content processing is a pipeline/resource pressure axis and should stay separate from simulation, model systems, and synthesis systems.

## Upward Consumers

- `engine_runtime`
- `engine_startup`
- `engine_generation`
- `engine_imaging`
- `engine_acoustics`

## Downward Dependencies

- `engine_core` — Base contracts.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Ingest | Source-side ingest model. | `40_INGEST.md` |
| Transform | Resource transform and preparation model. | `41_TRANSFORM.md` |
| Metadata | Metadata extraction and descriptor model. | `42_METADATA.md` |
| Resource Products | Canonical products emitted by content processing. | `43_RESOURCE_PRODUCTS.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.
