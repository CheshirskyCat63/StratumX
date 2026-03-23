# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_startup` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_world` | World truth boundary. |
| `engine_runtime` | Runtime constitution. |
| `engine_runtime_headless` | Headless runtime profile. |
| `engine_runtime_realtime` | Realtime runtime profile. |
| `engine_kinetics` | Critical simulation family wiring. |
| `engine_field` | Critical simulation family wiring. |
| `engine_agents` | Critical simulation family wiring. |
| `engine_inference` | Model system wiring. |
| `engine_generation` | Model system wiring. |
| `engine_imaging` | Synthesis system wiring. |
| `engine_acoustics` | Synthesis system wiring. |
| `engine_content` | Resource system wiring. |

## Downward Pattern

`engine_startup` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.
