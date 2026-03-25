# playtest_and_capture_operations Level

Canonical layer: `playtest_and_capture_operations`
Activation class: `cold-session`.

## Owns
- playtest session launcher, bug capture, screenshot/video capture, repro bundles, and telemetry review

## Consumes
- play/simulate surfaces, diagnostics, and build outputs

## Emits
- playtest launch/capture requests

## Never owns
- simulation truth
