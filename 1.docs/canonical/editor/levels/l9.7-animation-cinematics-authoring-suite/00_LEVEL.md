# animation_cinematics_authoring_suite Level

Canonical layer: `animation_cinematics_authoring_suite`
Activation class: `warm-suite`.

## Role
animation cinematics authoring suite is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- timeline/sequencer surfaces, camera tools, shot management, event tracks, animation diagnostics

## Consumes
- animation/cinematic projections, viewport previews, asset projections

## Emits
- animation/cinematic edit requests, shot and track requests, preview requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- animation truth
