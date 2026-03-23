# engine_startup

## Stack Position

L4. Startup

## Primary Role

Startup and assembly.

## Canonical Scope

Bootstrap, profile selection, runtime wiring, and tracing initialization.

## Boundary Rationale

Startup is the highest engine-owned layer because it selects the runtime profile, wires families, and launches execution without becoming part of world truth or runtime law.

## Upward Consumers

- No engine-owned consumer above this layer. This crate terminates at the top of the engine stack.

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_world` — World truth boundary.
- `engine_runtime` — Runtime constitution.
- `engine_runtime_headless` — Headless runtime profile.
- `engine_runtime_realtime` — Realtime runtime profile.
- `engine_kinetics` — Critical simulation family wiring.
- `engine_field` — Critical simulation family wiring.
- `engine_agents` — Critical simulation family wiring.
- `engine_inference` — Model system wiring.
- `engine_generation` — Model system wiring.
- `engine_imaging` — Synthesis system wiring.
- `engine_acoustics` — Synthesis system wiring.
- `engine_content` — Resource system wiring.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Bootstrap | Engine bootstrap sequence. | `40_BOOTSTRAP.md` |
| Profile Selection | Runtime profile selection model. | `41_PROFILE_SELECTION.md` |
| Runtime Wiring | Wiring of runtime and family surfaces. | `42_RUNTIME_WIRING.md` |
| Tracing Initialization | Startup tracing and diagnostics initialization. | `43_TRACING_INITIALIZATION.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.
