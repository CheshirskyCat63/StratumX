# inspector_system Level

Canonical layer: `inspector_system`
Activation class: `warm-view`.

## Role
inspector system is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- typed details panels, staged edit widgets, validation hints, provenance and dependency projections

## Consumes
- focused object projections, staged-edit state, diagnostics hints, assistant context hooks

## Emits
- staged edit requests, preview requests, assistant invoke requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- authoritative values
