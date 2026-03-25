# viewport_system Level

Canonical layer: `viewport_system`
Activation class: `warm-viewport`.

## Owns
- viewport hosts, camera state, render mode controls, gizmo channels, viewport tabs, and overlay attachment points

## Consumes
- snapshots, preview results, diagnostics, and active tool contexts

## Emits
- viewport interaction requests and visual mode changes

## Never owns
- engine or editor truth
