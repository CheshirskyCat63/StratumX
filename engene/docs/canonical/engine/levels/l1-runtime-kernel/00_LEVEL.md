# L1. Runtime Kernel

## Level Role

Execution ownership, phase control, scheduling, and apply governance.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_runtime` | Execution constitution and global parallel ownership. |
| `engine_runtime_headless` | Headless simulation runtime profile. |
| `engine_runtime_realtime` | Realtime interactive runtime profile. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `runtime/`
- `runtime-headless/`
- `runtime-realtime/`
