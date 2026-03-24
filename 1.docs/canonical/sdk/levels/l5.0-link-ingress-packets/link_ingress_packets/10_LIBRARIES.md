# Libraries

## Required external libraries
- `serde`
- `smallvec`
- `thiserror`

## Conditionally allowed libraries
- `tracing`

## Unjustified or forbidden libraries
- `crossbeam`
- GUI libraries
- engine internals beyond public L4 packet surfaces

## Rationale
Pure packet boundary layer.
