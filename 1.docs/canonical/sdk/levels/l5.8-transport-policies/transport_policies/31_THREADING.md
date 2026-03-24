# Threading

## Canonical threading model
low-churn single-writer immutable policy publication

## Local concurrency notes
Publication for this layer obeys the package threading class for policy layer.

## Forbidden concurrency drift
- no hidden scheduler ownership
- no UI event-loop coupling
- no mutable cross-layer mega-state
