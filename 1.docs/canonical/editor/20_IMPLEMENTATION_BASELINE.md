# Implementation Baseline

## UI baseline
- Rust-native
- `egui`-compatible product shell and panels
- lower-stack independence from UI toolkit
- multi-viewport capable host
- command routing via typed lower-stack requests

## Product baseline
- docked shell
- at least one world/scene viewport
- content browser
- outliner
- details inspector
- diagnostics/output
- build center
- assistant dock
- saved workspace layouts
- suite activation and overlays

## Code law
`editor/` may depend on lower-stack public surfaces only.
No UI code may import lower-stack internals by backdoor.
