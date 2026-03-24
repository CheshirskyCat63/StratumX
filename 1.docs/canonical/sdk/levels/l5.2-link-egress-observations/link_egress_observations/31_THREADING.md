# Threading

## Canonical threading model
immutable fanout publication with single canonical boundary ownership

## Local concurrency notes
Publication for this layer obeys the package threading class for boundary layer.

## Forbidden concurrency drift
- no hidden scheduler ownership
- no UI event-loop coupling
- no mutable cross-layer mega-state
