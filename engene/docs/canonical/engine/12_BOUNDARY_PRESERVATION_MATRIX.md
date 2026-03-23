# Boundary Preservation Matrix

## Purpose

This document records which earlier boundaries are preserved, widened, repositioned, or removed in the master canon.

## Matrix

| Earlier Boundary | Status in Master Canon | Preservation Form |
|---|---|---|
| `engine_collision` | preserved | internal element `40_COLLISION.md` inside `engine_kinetics` |
| `engine_rigidbody` | preserved | internal element `41_RIGIDBODY.md` inside `engine_kinetics` |
| `engine_ballistics` | widened | split into `42_TRAJECTORY.md` and `43_IMPACT.md` inside `engine_kinetics` |
| `engine_fluids` | preserved | internal element `40_FLUID_FIELD.md` inside `engine_field` |
| `engine_fire` | widened | preserved through `41_THERMAL_FIELD.md` inside `engine_field` |
| `engine_terrain` | preserved | internal element `42_TERRAIN_FIELD.md` inside `engine_field` |
| `engine_destruction` | widened | preserved through `43_STRUCTURAL_FIELD.md` inside `engine_field` |
| `engine_weather` | widened | preserved through `44_ATMOSPHERIC_FIELD.md` inside `engine_field` |
| `engine_navigation` | preserved | internal element `40_NAVIGATION.md` inside `engine_agents` |
| `engine_perception` | preserved | internal element `41_PERCEPTION.md` inside `engine_agents` |
| `engine_ai` | widened | preserved through `42_DECISION.md` inside `engine_agents` |
| `engine_social` | widened | preserved through `43_SOCIETY.md` inside `engine_agents` |
| `engine_schedule` | preserved | internal element `44_SCHEDULE.md` inside `engine_agents` |
| `engine_dialogue` | removed as standalone crate | absorbed into `engine_generation`; dialogue is one use-case, not the crate boundary |
| `engine_render` | widened | preserved through `engine_imaging` as image synthesis family |
| `engine_audio` | widened | preserved through `engine_acoustics` as acoustic synthesis family |
| `engine_material` | repositioned | preserved as `L0.5` shared world property substrate |
| `engine_content` | repositioned | preserved as `L3.2` resource system |
| `engine_inference` | repositioned | preserved as `L3.0` model system |
| `engine_startup` | repositioned | preserved as `L4` startup |

## Reading Rule

A widened boundary preserves the earlier capability class but places it under a larger and more general canonical crate boundary.

A repositioned boundary preserves the crate but changes its level placement in order to better reflect execution cost, ownership type, or substrate role.
