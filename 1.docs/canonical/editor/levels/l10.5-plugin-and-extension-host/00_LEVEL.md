# plugin_and_extension_host Level

Canonical layer: `plugin_and_extension_host`
Activation class: `cold-service`.

## Owns
- plugin registry, capability sandbox, extension lifecycle, API exposure, and extension review surfaces

## Consumes
- package metadata, lower-stack public APIs, and diagnostics

## Emits
- plugin load/unload/install requests

## Never owns
- hidden mutation power
