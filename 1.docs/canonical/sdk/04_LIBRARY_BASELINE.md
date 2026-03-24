# Library Baseline

## Global baseline allowed in L5
- `serde`
- `thiserror`
- `smallvec`
- `indexmap`
- `semver`
- `tracing`
- `bitflags`

## Global restrictions
- GUI libraries are forbidden in L5.
- Render backends are forbidden in L5.
- Content-import libraries are forbidden in L5.
- Assistant/provider libraries are forbidden in L5.
- Heavy workflow runtimes are forbidden in L5.
- Serialization libraries are allowed only where the owned data kind needs external persistence or interchange; they are not automatically justified for boundary hot in-process transfers.

## Per-layer rule
A library may appear in a layer only if:
- it is required by the owned data kind, or
- it is conditionally justified by the local contract

Anything not locally justified is not canonically allowed merely because it exists in the global baseline.
