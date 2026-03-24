# Threading

## Canonical threading model
ordered single-writer submission per engine session and per packet stream

## Local concurrency notes
Publication for this layer obeys the package threading class for boundary layer.

## Forbidden concurrency drift
- no hidden scheduler ownership
- no UI event-loop coupling
- no mutable cross-layer mega-state
