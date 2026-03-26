# Play, Simulate, and Debug Model

## Product surfaces
- play controls
- simulate controls
- pause/step controls
- debug visual toggles
- trace, event, and metrics projections
- runtime/view sync markers

## Laws
- play/simulate state is a surface-level controller above lower-stack runtime boundaries
- debug overlays are read-mostly and discardable
- runtime transitions must remain explicit and reversible
