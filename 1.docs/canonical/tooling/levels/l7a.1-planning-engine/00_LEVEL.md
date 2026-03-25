# planning_engine Level

Canonical layer: `planning_engine`
Activation class: `cold`.

## Owns
- planning IR, step graphs, checkpoints, fallback branches, and sequencing

## Consumes
- normalized goals and canon constraints

## Emits
- plan bundles

## Never owns
- apply authority
