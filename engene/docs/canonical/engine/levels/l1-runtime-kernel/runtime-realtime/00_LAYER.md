# engine_runtime_realtime

## Stack Position

L1. Runtime Kernel

## Primary Role

Realtime interactive runtime profile.

## Canonical Scope

Realtime profile wiring, frame cadence, and realtime execution outputs.

## Boundary Rationale

Realtime execution needs its own profile so interactive cadence and latency-sensitive orchestration stay explicit without polluting headless execution.

## Upward Consumers

- `engine_startup`

## Downward Dependencies

- `engine_runtime` — Execution constitution.
- `engine_world` — World truth boundary.
- `engine_ecs` — ECS substrate.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Realtime Profile | Canonical realtime profile definition. | `40_REALTIME_PROFILE.md` |
| Frame Cadence | Realtime frame/tick cadence model. | `41_FRAME_CADENCE.md` |
| Realtime Outputs | Outputs of realtime execution. | `42_REALTIME_OUTPUTS.md` |
