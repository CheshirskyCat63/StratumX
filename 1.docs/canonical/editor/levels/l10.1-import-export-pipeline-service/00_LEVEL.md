# import_export_pipeline_service Level

Canonical layer: `import_export_pipeline_service`
Activation class: `batch-service`.

## Role
import export pipeline service is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- import jobs, export jobs, reimport jobs, conversion status, dependency-aware pipeline orchestration

## Consumes
- content browser requests, asset metadata, package/dependency service, diagnostics

## Emits
- import/export job requests, pipeline status, artifact update requests

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- shell state or asset browser truth
