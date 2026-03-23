# Communication

## Inputs

World snapshots, ECS-extracted scene data, material descriptors, spatial context, and runtime-issued synthesis windows.

## Outputs

Image synthesis products, extraction products, visibility products, and diagnostics.

## Canonical Data Forms

- scene extraction
- visibility products
- lighting products
- image outputs

## Upward Flow

`engine_imaging` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.
