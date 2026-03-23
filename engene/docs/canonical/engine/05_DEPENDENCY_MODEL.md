# Dependency Model

## Canonical Dependency Overview

- `engine_core` -> none
- `engine_identity` -> engine_core
- `engine_handle` -> engine_core, engine_identity
- `engine_storage_layout` -> engine_core
- `engine_storage_access` -> engine_core, engine_handle, engine_storage_layout
- `engine_storage_mutation` -> engine_core, engine_handle, engine_storage_layout, engine_storage_access
- `engine_ecs_registry` -> engine_core, engine_identity, engine_handle, engine_storage_layout
- `engine_ecs_query` -> engine_core, engine_identity, engine_handle, engine_storage_access, engine_ecs_registry
- `engine_ecs` -> engine_core, engine_identity, engine_handle, engine_storage_layout, engine_storage_access, engine_storage_mutation, engine_ecs_registry, engine_ecs_query
- `engine_world_spatial` -> engine_core, engine_identity, engine_handle
- `engine_world_region` -> engine_core, engine_world_spatial
- `engine_world` -> engine_core, engine_handle, engine_ecs, engine_world_spatial, engine_world_region
- `engine_material` -> engine_core, engine_handle, engine_world
- `engine_runtime` -> engine_core, engine_handle, engine_ecs, engine_world
- `engine_runtime_headless` -> engine_runtime, engine_world, engine_ecs
- `engine_runtime_realtime` -> engine_runtime, engine_world, engine_ecs
- `engine_kinetics` -> engine_core, engine_world, engine_material, engine_world_spatial, engine_world_region
- `engine_field` -> engine_core, engine_world, engine_material, engine_world_spatial, engine_world_region
- `engine_agents` -> engine_core, engine_ecs, engine_world, engine_world_region
- `engine_inference` -> engine_core, engine_world, engine_ecs
- `engine_generation` -> engine_core, engine_world, engine_inference
- `engine_imaging` -> engine_core, engine_world, engine_ecs, engine_material, engine_world_spatial, engine_world_region
- `engine_acoustics` -> engine_core, engine_world, engine_ecs, engine_material, engine_world_spatial, engine_world_region
- `engine_content` -> engine_core
- `engine_startup` -> engine_core, engine_world, engine_runtime, engine_runtime_headless, engine_runtime_realtime, engine_kinetics, engine_field, engine_agents, engine_inference, engine_generation, engine_imaging, engine_acoustics, engine_content

The stack is downward-dependent: every higher layer consumes lower substrates or higher-root families without reversing world ownership or execution ownership.
