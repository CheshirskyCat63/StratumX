# Communication

## Inputs

World snapshots, ECS substrate, commands, intents, time state, and registered family surfaces.

## Outputs

Execution progression, scheduled execution windows, staged apply ordering, events, and diagnostics.

## Canonical Data Forms

- phase contracts
- execution windows
- staged outputs
- events
- diagnostics

## Upward Flow

`engine_runtime` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.
