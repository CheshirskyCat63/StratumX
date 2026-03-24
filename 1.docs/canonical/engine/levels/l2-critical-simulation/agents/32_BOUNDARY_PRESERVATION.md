# Boundary Preservation

## Preserved Earlier Boundaries

The following earlier standalone boundaries remain explicit inside `engine_agents`:
- `engine_navigation`
- `engine_perception`
- `engine_schedule`

## Widened Earlier Boundaries

The following earlier boundaries are preserved as widened internal elements:
- `engine_ai` -> `Decision`
- `engine_social` -> `Society`

## Reason

The family boundary keeps agent and society computation in one execution class while preserving exact internal functional splits.
