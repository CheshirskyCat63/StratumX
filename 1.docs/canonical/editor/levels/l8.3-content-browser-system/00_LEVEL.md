# content_browser_system Level

Canonical layer: `content_browser_system`
Activation class: `warm-view`.

## Role
content browser system is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- asset browser views, source/imported/cooked asset projections, search/filter state, tags, dependency projections, thumbnail hosts

## Consumes
- asset indices, metadata projections, service-backed import/export status, validation signals

## Emits
- asset open/import/reimport requests, selection/focus requests, batch service requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- asset truth, import pipeline truth
