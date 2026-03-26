# package_market_and_dependency_service Level

Canonical layer: `package_market_and_dependency_service`
Activation class: `cold-service`.

## Role
package market and dependency service is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- package registry views, dependency graphs, compatibility checks, source retrieval status

## Consumes
- plugin host, bootstrap service, import/export service, diagnostics

## Emits
- package install/update/remove requests, dependency results

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- editor truth or asset truth
