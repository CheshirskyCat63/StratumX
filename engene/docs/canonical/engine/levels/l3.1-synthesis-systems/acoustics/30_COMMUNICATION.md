# Communication

## Inputs

World snapshots, source state, material descriptors, spatial context, and runtime-issued synthesis windows.

## Outputs

Propagation products, acoustic field products, output-ready acoustic products, and diagnostics.

## Canonical Data Forms

- propagation products
- reflection/occlusion products
- material response products
- acoustic outputs

## Upward Flow

`engine_acoustics` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.
