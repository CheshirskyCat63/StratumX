# Boundary Preservation

## Preserved Earlier Boundaries

`engine_collision` and `engine_rigidbody` are preserved as explicit internal boundaries.

## Widened Earlier Boundary

`engine_ballistics` is widened and split into:
- `Trajectory`
- `Impact`

This preserves ballistic computation while separating flight and impact concerns.

## Reason

The family boundary is narrower in ownership type and clearer in execution cost than three standalone crates.
