# Tool Context and Mode Model

## Global tools
- select
- move
- rotate
- scale
- pivot
- measure
- annotate
- bookmark
- isolate / visibility
- simulation controls

## Context tools
- terrain sculpt / smooth / flatten / erosion / paint
- foliage / vegetation placement
- spline and road tools
- fracture / cluster / support graph tools
- fluid/fire/weather emitters and zones
- AI zone and navigation tools
- material painting and lookdev tools
- animation rigging and sequencing tools
- UI layout tools
- logic graph tools

## Mode rule
Tool contexts are explicit.
Only one dominant manipulation mode may own the active gizmo channel at a time.
Tool contexts may add overlays and side panels but may not hijack lower-stack authority.
