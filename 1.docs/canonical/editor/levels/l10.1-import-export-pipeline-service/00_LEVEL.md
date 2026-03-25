# import_export_pipeline_service Level

Canonical layer: `import_export_pipeline_service`
Activation class: `cold-service`.

## Owns
- importers, exporters, transcoders, reimport flows, external DCC handoff, and asset source tracking

## Consumes
- content browser surfaces, artifact plane, and diagnostics

## Emits
- import/export requests and staged conversion jobs

## Never owns
- asset authority
