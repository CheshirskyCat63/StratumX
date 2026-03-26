# Global Dependency Model

## Legal Package Dependencies

The following dependencies are legal within the StratumX canonical stack:

| Dependent Package | Dependency Package | Dependency Type | Notes |
|-------------------|--------------------|-----------------|-------|
| sdk | engine | typed bridge | sdk consumes engine truth through defined interfaces only |
| tooling | sdk | orchestration substrate | tooling uses sdk as typed bridge to engine |
| tooling | engine | indirect only | tooling must not read engine truth directly; must go through sdk |
| editor | tooling | authoring shell | editor consumes tooling orchestration and assistant runtimes |
| editor | sdk | indirect only | editor must not read sdk truth directly; must go through tooling for orchestration |
| editor | engine | forbidden | editor must never read engine truth directly or indirectly through illegal paths |

## Illegal Direct Dependencies

The following direct dependencies are prohibited:

- editor -> engine (any form of truth read)
- editor -> sdk (any form of truth read)
- tooling -> engine (any form of truth read)
- sdk -> tooling (as truth owner; sdk is only a bridge)
- sdk -> editor (sdk must not consume editor)
- engine -> sdk (as truth consumer; engine owns truth and does not consume)
- engine -> tooling (engine must not consume tooling)
- engine -> editor (engine must not consume editor)

## Dependency Direction Rules

1. Truth flows upward: engine -> sdk -> tooling -> editor
2. Control flows downward: editor -> tooling -> sdk -> engine (only through legal interfaces)
3. No package may own truth of another package
4. sdk is strictly a typed bridge, not a second truth source
5. All cross-package reads must use explicitly defined request/result/ref classes
6. No package may bypass the dependency chain to read truth

## Exact Allowed Dependency Direction Table

| From \ To | engine | sdk | tooling | editor |
|-----------|--------|-----|---------|--------|
| engine    | -      | X   | X       | X      |
| sdk       | L      | -   | L       | X      |
| tooling   | X      | L   | -       | L      |
| editor    | X      | X   | L       | -      |

Where:
- L = legal dependency (through defined interfaces only)
- X = illegal dependency (prohibited)
- - = self-dependency (not applicable)

## Interface Contracts

Legal inter-package surfaces are limited to:
- engine -> sdk: request/result/ref classes defined in `sdk/26_SHARED_TYPE_REGISTRY.md`, `sdk/16_BOUNDARY_AUTHORITY.md`, and `sdk/31_ENGINE_L4_BINDING_MAP.md`
- sdk -> tooling: request/result/ref classes defined in `tooling/19_CROSS_LAYER_EXCHANGE_MODEL.md`, `tooling/26_SHARED_TYPE_REGISTRY.md`, and `tooling/22_L5_SYNCHRONIZATION_MODEL.md`
- tooling -> editor: request/result/ref classes defined in `editor/22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`, `editor/31_SHARED_TYPE_REGISTRY.md`, and `editor/33_BOUNDARY_PRESERVATION_MATRIX.md`

No other surfaces are permitted for truth transfer.