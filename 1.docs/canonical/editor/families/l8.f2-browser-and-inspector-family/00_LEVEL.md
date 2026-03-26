# browser_and_inspector_family Family

Canonical family: `browser_and_inspector_family`.

## Role
This family groups related editor product surfaces or services that share data locality, activation regime, and request/view discipline.

## Owns
- outliner, content browser, details/inspector, and metadata-heavy product surfaces

## Consumes
- snapshot and index projections, diagnostics hints

## Emits
- focus, reveal, staged edit, and browser requests

## Family law
Members of this family cohere by data type, lifetime, and invalidation regime.
They may share indices, projections, and activation policy where legal.

## Never owns
- authoritative domain state
