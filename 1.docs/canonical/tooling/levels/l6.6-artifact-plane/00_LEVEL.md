# artifact_plane Level

Canonical layer: `artifact_plane`
Activation class: `warm-build`.

## Owns
- deterministic generated products and artifact manifests

## Consumes
- snapshots, build requests, policies

## Emits
- artifact manifests, packageable units, diagnostics surfaces

## Never owns
- caches disguised as artifacts or mutable truth
