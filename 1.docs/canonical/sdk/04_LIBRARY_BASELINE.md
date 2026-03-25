# Library Baseline

Allowed library classes:
- transport-safe serialization libraries
- bounded queue and ring-buffer libraries
- immutable view and small-vector style helpers
- fixed-schema versioning and validation helpers

Forbidden library classes:
- hidden databases
- hidden disk caches
- hidden model runtimes
- auto-growing graph stores
- uncontrolled GPU ownership
- reflection-heavy dynamic registries in hot publication paths

L5 libraries must help move typed data, not invent new state.
