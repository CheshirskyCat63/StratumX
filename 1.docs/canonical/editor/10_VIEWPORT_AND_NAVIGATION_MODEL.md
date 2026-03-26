# Viewport and Navigation Model

The viewport is the dominant interaction surface of the editor.
It hosts camera navigation, transform and domain gizmos, overlays, play/simulate toggles, debug visuals, and preview feedback.

## Viewport responsibilities
- 3D scene and world rendering
- camera navigation and framing
- hit-testing and selection routing
- manipulator visualization
- mode-specific overlays
- playback and simulation controls
- debug and diagnostics overlays

## Navigation law
Viewport navigation state is product-local and ephemeral.
It never owns lower-stack truth.

## Multi-viewport law
Multiple viewports may exist, but only focused viewports are hot.
Inactive viewports degrade refresh rate or become cold.
