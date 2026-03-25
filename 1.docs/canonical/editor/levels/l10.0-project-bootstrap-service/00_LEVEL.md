# project_bootstrap_service Level

Canonical layer: `project_bootstrap_service`
Activation class: `cold-service`.

## Owns
- new project templates, project migration entrypoints, starter pack composition, and bootstrap validation

## Consumes
- template registry and lower-stack project surfaces

## Emits
- project creation and bootstrap requests

## Never owns
- active runtime truth
