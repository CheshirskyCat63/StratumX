# Canon Transition Map

## Purpose

This document freezes the transition from the earlier split lower/upper canonical packages into the current unified master canon.

It exists to prevent naming drift, boundary drift, and undocumented reinterpretation of crate roles.

## Rules

- The master canon is authoritative.
- Earlier package names remain valid only as historical references unless explicitly preserved here.
- A changed boundary is canonical only when its preservation or widening is documented here.

## Level Transitions

| Earlier Canon | Master Canon | Transition Type | Canonical Meaning |
|---|---|---|---|
| `L2. Physical Domains` | `L0.5 + L2` | repartition | shared physical properties moved to `L0.5`; critical physical compute remains in `L2` |
| `L2. Behavioral Domains` | `L2. Critical Simulation Families` | merge and regroup | behavioral compute is preserved inside `engine_agents` |
| `L2. Inference and Dialogue Domains` | `L3.0. Model Systems` | regroup and widen | model-facing tasks move above critical simulation |
| `L2. Presentation and Resource Domains` | `L3.1 + L3.2` | regroup and split | synthesis and resource processing are separated |
| `L3. Startup` | `L4. Startup` | level shift | startup remains the highest launch layer |

## Crate Boundary Transitions

| Earlier Boundary | Master Boundary | Transition Type | Canonical Note |
|---|---|---|---|
| `engine_material` | `engine_material` | repositioned | moved from regular domain position to shared world property substrate at `L0.5` |
| `engine_collision` + `engine_rigidbody` + `engine_ballistics` | `engine_kinetics` | merged and renamed | preserved as one local dynamics family with collision, rigidbody, trajectory, and impact internal boundaries |
| `engine_fluids` + `engine_fire` + `engine_terrain` + `engine_destruction` + `engine_weather` | `engine_field` | merged and renamed | preserved as one distributed field family with fluid, thermal, terrain, structural, and atmospheric internal boundaries |
| `engine_navigation` + `engine_perception` + `engine_ai` + `engine_social` + `engine_schedule` | `engine_agents` | merged and renamed | preserved as one agent and society family with explicit internal boundaries |
| `engine_inference` | `engine_inference` | repositioned | moved from L2 to `L3.0` as a model system |
| `engine_dialogue` | `engine_generation` | widened replacement | dialogue is not preserved as a standalone crate; generative output work is widened into `engine_generation` |
| `engine_render` | `engine_imaging` | widened replacement | render-facing concerns are preserved and widened into image synthesis |
| `engine_audio` | `engine_acoustics` | widened replacement | audio-facing concerns are preserved and widened into acoustic synthesis |
| `engine_content` | `engine_content` | repositioned | moved from L2 presentation/resource grouping to `L3.2. Resource Systems` |
| `engine_startup` | `engine_startup` | level shift | moved from earlier L3 to canonical `L4` |

## Lower-Stack Renames

| Earlier File Name | Master File Name | Transition Type |
|---|---|---|
| `40_MATH.md` | `40_MATH_BACKBONE.md` | clarified rename |
| `41_TYPES.md` | `41_BASE_TYPES.md` | clarified rename |
| `42_ERRORS.md` | `42_ERROR_MODEL.md` | clarified rename |
| `43_TRAITS.md` | `43_CORE_CONTRACTS.md` | clarified rename |
| `41_VALIDATION.md` | `41_VALIDATION_MODEL.md` | clarified rename |
| `42_INVALIDATION.md` | `42_INVALIDATION_MODEL.md` | clarified rename |
| `41_WRITE_VIEWS.md` | `41_WRITE_WINDOWS.md` | clarified rename |

## Preservation Rule

When an earlier standalone crate is merged into a family crate, its role must remain visible through:
- explicit internal element documents;
- explicit dependency baselines;
- explicit boundary preservation notes.

The master canon follows this rule for `engine_kinetics`, `engine_field`, `engine_agents`, `engine_generation`, `engine_imaging`, and `engine_acoustics`.
