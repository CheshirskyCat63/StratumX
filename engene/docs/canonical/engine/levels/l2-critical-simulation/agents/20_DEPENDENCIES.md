# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_agents` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_ecs` | ECS substrate. |
| `engine_world` | World truth boundary. |
| `engine_world_region` | Region substrate. |

## Downward Pattern

`engine_agents` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.
