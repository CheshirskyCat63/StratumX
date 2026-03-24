# Threading

## Canonical threading model
parallel evaluation with singular immutable verdict publication

## Local concurrency notes
Publication for this layer obeys the package threading class for verdict layer.

## Forbidden concurrency drift
- no hidden scheduler ownership
- no UI event-loop coupling
- no mutable cross-layer mega-state
