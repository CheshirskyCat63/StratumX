# plugin_and_extension_host Level

Canonical layer: `plugin_and_extension_host`
Activation class: `warm-service`.

## Role
plugin and extension host is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- plugin registry, extension discovery, extension capability binding, tool/panel/service registration surfaces

## Consumes
- package/dependency service, editor shell/service registries, diagnostics

## Emits
- extension load/unload requests, capability announcements, legality results

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- lower-stack authority or hidden plugin-owned truths
