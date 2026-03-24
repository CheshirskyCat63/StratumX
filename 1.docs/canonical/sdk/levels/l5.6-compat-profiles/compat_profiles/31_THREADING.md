# Threading

## Canonical threading model
low-churn single-writer fact publication

## Local concurrency notes
Publication for this layer obeys the package threading class for fact layer.

## Forbidden concurrency drift
- no hidden scheduler ownership
- no UI event-loop coupling
- no mutable cross-layer mega-state
