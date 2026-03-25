# Dependency Model

## Allowed dependency direction
- `L6 -> L5`
- `L6A -> L6`
- `L6A -> bounded L7A planning services`
- `L7 -> L6` by explicit compiled campaign and task surfaces only
- `L7A -> L6A`
- `L7A -> bounded L7 context services`

## Forbidden dependency direction
- `L6 -> L6A` for hidden assistant ownership
- `L6 -> L7` for hidden orchestration ownership
- `L6 -> L7A` for hidden planning ownership
- `L6A -> engine` direct access
- `L7 -> engine` direct access
- `L7A -> L6` direct transaction ownership
- `L7A -> engine` direct access

## Dependency law
Upper layers may request, plan, validate, optimize, or orchestrate, but only `L6` may own editor mutation authority.
The only always-hot dependency path is `engine/L5 -> L6`.
`L7` and `L7A` are cold compiled-control layers and must stay off ordinary frame-level editor work.
