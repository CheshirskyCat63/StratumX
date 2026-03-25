# Viewport and Navigation Model

## Viewport requirements
- perspective and orthographic modes
- multiple simultaneous viewports
- camera bookmarks
- framing and focus
- visibility filters
- lighting and debug visualization modes
- transform gizmos
- local/world space switching
- snapping controls
- simulation overlay controls
- streaming/partition/debug overlays
- large-world navigation support

## Viewport modes
- edit
- play-in-editor
- simulate
- cinematic
- diagnostics
- authoring-suite specific mode overlays

## Navigation law
Viewport navigation is direct and low-latency.
Heavy suite logic may annotate the viewport but may not stall camera movement or core manipulation.
