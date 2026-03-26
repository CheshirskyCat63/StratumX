# assistant_diagnostics_build_family Family

Canonical family: `assistant_diagnostics_build_family`.

## Role
This family groups related editor product surfaces or services that share data locality, activation regime, and request/view discipline.

## Owns
- assistant, diagnostics, and build/release auxiliary surfaces

## Consumes
- assistant/runtime projections, diagnostics streams, build/release status

## Emits
- assistant/build/diagnostics requests

## Family law
Members of this family cohere by data type, lifetime, and invalidation regime.
They may share indices, projections, and activation policy where legal.

## Never owns
- assistant runtime or build truth
