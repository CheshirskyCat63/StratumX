# migration_planner Level

Canonical layer: `migration_planner`
Activation class: `cold`.

## Owns
- migration plans, staged transitions, rollback hints, and risk markers

## Consumes
- goals, canon constraints, current architecture context

## Emits
- migration plans

## Never owns
- editor mutation
