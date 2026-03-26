# viewport_and_manipulation_family Family

Canonical family: `viewport_and_manipulation_family`.

## Role
This family groups related editor product surfaces or services that share data locality, activation regime, and request/view discipline.

## Owns
- viewport hosts, navigation, overlays, gizmos, snapping, and focused manipulation context

## Consumes
- viewport projections, tool contexts, diagnostics overlays

## Emits
- manipulation and viewport requests

## Family law
Members of this family cohere by data type, lifetime, and invalidation regime.
They may share indices, projections, and activation policy where legal.

## Never owns
- world truth
