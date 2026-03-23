# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_field` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_world` | World truth boundary. |
| `engine_material` | Shared world property substrate. |
| `engine_world_spatial` | Spatial substrate. |
| `engine_world_region` | Region substrate. |

## Downward Pattern

`engine_field` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.
