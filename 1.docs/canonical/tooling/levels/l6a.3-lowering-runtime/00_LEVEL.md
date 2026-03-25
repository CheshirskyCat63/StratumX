# lowering_runtime Level

Canonical layer: `lowering_runtime`
Activation class: `warm`.

## Owns
- lowering IR, command bundle emission, dry-run lowering, and partial-failure mapping

## Consumes
- proposals, plan bundles, canon constraints, safety prechecks

## Emits
- typed L6 command bundles

## Never owns
- direct editor mutation
