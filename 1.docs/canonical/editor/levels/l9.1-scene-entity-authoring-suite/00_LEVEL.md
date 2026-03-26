# scene_entity_authoring_suite Level

Canonical layer: `scene_entity_authoring_suite`
Activation class: `warm-suite`.

## Role
scene entity authoring suite is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- entity placement, grouping, prefab/archetype views, transform workflows, scene diagnostics

## Consumes
- scene/entity projections, selection, inspector, viewport, validation hooks

## Emits
- scene/entity edit requests, spawn/place/duplicate/group requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- entity truth
