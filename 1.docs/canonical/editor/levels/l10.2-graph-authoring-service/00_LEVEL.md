# graph_authoring_service Level

Canonical layer: `graph_authoring_service`
Activation class: `cold-warm-service`.

## Owns
- graph canvas services, node schemas, validation helpers, search palettes, and graph serializers

## Consumes
- suite graph requests and lower-stack legality surfaces

## Emits
- graph edit requests and artifact production requests

## Never owns
- global truth
