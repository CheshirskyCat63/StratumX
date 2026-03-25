# transaction_ledger Level

Canonical layer: `transaction_ledger`
Activation class: `hot`.

## Owns
- ordered commit chain, undo/redo, apply/revert bindings, invalidation sets, transaction evidence

## Consumes
- accepted commands and authority deltas

## Emits
- transaction results, snapshot/index/artifact invalidations, stream publications

## Never owns
- bulk read state or hidden mutable mirrors
