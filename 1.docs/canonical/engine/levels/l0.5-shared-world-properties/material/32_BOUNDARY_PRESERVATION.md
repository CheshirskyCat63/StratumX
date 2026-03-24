# Boundary Preservation

## Canonical Position

`engine_material` is preserved as a canonical crate and moved out of the regular simulation tier.

## Earlier Boundary Relation

Earlier canon treated `engine_material` as a physical-domain crate. Master canon keeps the crate but reclassifies it as `L0.5. Shared World Property Substrate`.

## Reason

`engine_material` is read-heavy shared substrate consumed by `engine_kinetics`, `engine_field`, `engine_imaging`, and `engine_acoustics`. It is not a regular world-progression family on its own.
