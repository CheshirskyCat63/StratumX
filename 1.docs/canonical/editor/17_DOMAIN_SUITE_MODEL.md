# Domain Suite Model

The editor product ships with broad domain suites rather than one giant monolithic mode.

## Required suites
- world authoring
- scene/entity authoring
- terrain/landscape
- material/lookdev
- destruction/fracture
- simulation/AI
- weather/environment
- animation/cinematics
- audio/voice
- UI/HUD
- quest/event/logic
- build/validation/release

## Suite law
Each suite is warm-on-demand.
Each suite declares:
- its primary views
- its viewport overlays
- its inspectors
- its command classes
- its preview classes
- its diagnostics classes
- its build artifact classes
