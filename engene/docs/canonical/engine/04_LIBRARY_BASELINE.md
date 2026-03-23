# External Library Baseline

| Crate | External Dependencies |
|---|---|
| engine_core | glam, serde, thiserror |
| engine_identity | slotmap, serde |
| engine_handle | slotmap, serde |
| engine_storage_layout | smallvec, bitflags, serde |
| engine_storage_access | smallvec, bitflags |
| engine_storage_mutation | smallvec, bitflags |
| engine_ecs_registry | serde |
| engine_ecs_query | smallvec, bitflags |
| engine_ecs | serde |
| engine_world_spatial | glam, serde |
| engine_world_region | serde, bitflags |
| engine_world | serde, bincode, glam |
| engine_material | serde, bitflags, smallvec |
| engine_runtime | tracing, rayon, crossbeam, parking_lot, smallvec |
| engine_runtime_headless | tracing, smallvec |
| engine_runtime_realtime | tracing, smallvec |
| engine_kinetics | rapier3d, glam, serde, bitflags, smallvec, tracing |
| engine_field | glam, serde, bitflags, smallvec, tracing |
| engine_agents | glam, serde, bitflags, smallvec, tracing |
| engine_inference | serde, serde_json, thiserror, smallvec, tracing |
| engine_generation | serde, serde_json, thiserror, smallvec, tracing |
| engine_imaging | wgpu, bytemuck, glam, serde, smallvec, tracing |
| engine_acoustics | glam, serde, smallvec, bitflags, tracing |
| engine_content | walkdir, image, gltf, serde, thiserror, smallvec, tracing |
| engine_startup | tracing, tracing-subscriber, serde, serde_json, backtrace |

Each crate-level `10_LIBRARIES.md` document defines exact role mapping for every dependency.
