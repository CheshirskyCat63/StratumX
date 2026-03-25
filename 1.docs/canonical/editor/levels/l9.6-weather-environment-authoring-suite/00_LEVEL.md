# weather_environment_authoring_suite Level

Canonical layer: `weather_environment_authoring_suite`
Activation class: `warm-suite`.

## Owns
- sky, fog, rain, wind, wetness, probes, time-of-day, and environment presets

## Consumes
- weather/environment families and render/lookdev views

## Emits
- environment authoring requests

## Never owns
- weather truth
