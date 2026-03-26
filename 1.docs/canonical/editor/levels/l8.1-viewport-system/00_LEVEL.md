# viewport_system Level

Canonical layer: `viewport_system`
Activation class: `hot-when-focused`.

## Role
viewport system is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- viewport hosts, camera controllers, scene framing, hit-test presentation, viewport-local overlays, and active viewport markers

## Consumes
- scene and world projections, active tool context, overlay state, play/simulate/debug signals, preview projections

## Emits
- viewport interaction intents, focus changes, selection intents, tool interaction requests, play/simulate surface requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- world truth, simulation truth, build truth
