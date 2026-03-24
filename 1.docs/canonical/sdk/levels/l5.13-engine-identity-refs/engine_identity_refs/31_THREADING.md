# Threading

## Canonical threading model
immutable fanout identity publication with single canonical ownership

## Local concurrency notes
Publication for this layer obeys the package threading class for handle/ref layer.

## Forbidden concurrency drift
- no hidden scheduler ownership
- no UI event-loop coupling
- no mutable cross-layer mega-state
