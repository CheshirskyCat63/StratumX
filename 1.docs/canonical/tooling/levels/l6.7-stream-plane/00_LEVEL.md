# stream_plane Level

Canonical layer: `stream_plane`
Activation class: `hot-forward`.

## Owns
- bounded forward-only transaction, observation, metrics, diagnostics, and pressure streams

## Consumes
- L5 batches, transaction outputs, preview/build/release events

## Emits
- bounded readers and stream batches

## Never owns
- unbounded history or authority
