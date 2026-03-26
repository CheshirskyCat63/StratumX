# terrain_material_environment_family Family

Canonical family: `terrain_material_environment_family`.

## Role
This family groups related editor product surfaces or services that share data locality, activation regime, and request/view discipline.

## Owns
- terrain, lookdev, and environment suites

## Consumes
- terrain/material/environment projections, preview hooks

## Emits
- terrain/material/environment requests

## Family law
Members of this family cohere by data type, lifetime, and invalidation regime.
They may share indices, projections, and activation policy where legal.

## Never owns
- terrain/material/environment truth
