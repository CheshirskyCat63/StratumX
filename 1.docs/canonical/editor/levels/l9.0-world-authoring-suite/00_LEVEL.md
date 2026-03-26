# world_authoring_suite Level

Canonical layer: `world_authoring_suite`
Activation class: `warm-suite`.

## Role
world authoring suite is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- world partition surfaces, region/zone authoring views, streaming cell tools, world diagnostics, placement flows

## Consumes
- world projections, viewport system, outliner system, preview and validation hooks

## Emits
- world partition and world-structure edit requests, preview requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- world truth
