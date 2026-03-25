# script_and_hot_reload_service Level

Canonical layer: `script_and_hot_reload_service`
Activation class: `cold-warm-service`.

## Owns
- script editors, compile/reload surfaces, type discovery, reload health, and runtime script diagnostics

## Consumes
- build/runtime diagnostics and plugin host

## Emits
- compile/reload/script requests

## Never owns
- engine core authority
