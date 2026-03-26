# world_scene_family Family

Canonical family: `world_scene_family`.

## Role
This family groups related editor product surfaces or services that share data locality, activation regime, and request/view discipline.

## Owns
- world, scene, and entity authoring suites

## Consumes
- world and scene projections, viewport, inspector

## Emits
- world/scene/entity requests

## Family law
Members of this family cohere by data type, lifetime, and invalidation regime.
They may share indices, projections, and activation policy where legal.

## Never owns
- world or entity truth
