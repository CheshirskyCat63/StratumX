# workspace_runtime Level

Canonical layer: `workspace_runtime`
Activation class: `hot-ui`.

## Owns
- workspace session, layout, focus, selection presentation, panels, inspectors, overlays, and tool-host state

## Consumes
- snapshots, derived views, diagnostics, user input

## Emits
- workspace requests and view state

## Never owns
- domain truth or hidden bulk mirrors
