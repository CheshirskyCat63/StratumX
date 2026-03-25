# StratumX Editor Viewport and Interaction Constitution

The viewport is the primary manipulation surface.
It must remain low-latency and directly controllable.

## Laws
- camera motion and core gizmo manipulation are latency-sensitive
- overlays may annotate but not stall the viewport
- selection and edit requests must route through legal lower-stack paths
- suite-specific modes may extend viewport behavior only through declared tool contexts
