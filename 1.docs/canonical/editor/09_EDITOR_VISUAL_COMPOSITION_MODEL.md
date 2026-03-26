# Editor Visual Composition Model

## Primary composition
- top frame with menu bar, main toolbar, and status strip
- central viewport region
- left-side world/outliner region
- bottom or secondary content browser region
- right-side inspector/details region
- overlay bars and context shelves attached to active viewports
- optional diagnostics, assistant, and build/release docks

## Layout law
- all docked surfaces are layout-managed through `L8.7`
- no panel owns hidden domain truth
- closed panels are cold
- detached windows are secondary hosts, not separate truths

## Visual hierarchy
1. shell frame
2. active viewport and overlays
3. persistent browser/outliner/details anchors
4. context-sensitive panels and suites
5. diagnostics/assistant/build auxiliary surfaces
