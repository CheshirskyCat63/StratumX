# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_content` depends on it |
|---|---|
| `engine_core` | Base contracts. |

## Downward Pattern

`engine_content` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution boundary beneath its role.
