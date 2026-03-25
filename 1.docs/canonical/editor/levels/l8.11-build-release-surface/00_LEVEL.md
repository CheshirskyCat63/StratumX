# build_release_surface Level

Canonical layer: `build_release_surface`
Activation class: `warm-build`.

## Owns
- build queue view, release checklist, artifact browser, and packaging dashboards

## Consumes
- build runtime state, release runtime state, artifact views

## Emits
- build, package, and release requests

## Never owns
- build truth outside lower stack
