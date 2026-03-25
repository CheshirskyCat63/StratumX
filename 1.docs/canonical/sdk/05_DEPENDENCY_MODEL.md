# Dependency Model

Allowed direction:
- engine L4 publishes through L5
- upper tools consume engine exports through L5
- legality and compatibility evaluation may depend on exported facts only

Forbidden direction:
- L5 -> editor truth ownership
- L5 -> assistant runtime ownership
- L5 -> studio orchestration ownership
- L5 -> planning ownership
- L5 -> engine deep internals beyond public L4 bundles

L5 is a dependency funnel, not a dependency magnet.
