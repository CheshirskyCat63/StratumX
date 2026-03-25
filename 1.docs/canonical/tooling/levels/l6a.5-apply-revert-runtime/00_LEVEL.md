# apply_revert_runtime Level

Canonical layer: `apply_revert_runtime`
Activation class: `warm-guard`.

## Owns
- assistant apply/revert mediation, apply traces, revert bindings, and audit refs

## Consumes
- approved command bundles, transaction results

## Emits
- assistant-originated apply results and revert results

## Never owns
- mutation around the transaction ledger
