# Resource Discipline

## Product laws
- product surfaces are activation-bounded
- inactive suites are cold
- closed panels are cold
- hidden caches are forbidden
- equivalent data classes must co-reside by lifetime and invalidation regime
- panel state, suite state, and service job state must not be arbitrarily intermixed

## Concurrency laws
- UI state and focused interaction routing remain single-writer
- indexing, import/export, preview generation, validation scans, search, dependency analysis, and thumbnail generation may run in background jobs
- background jobs must be bounded, cancelable where practical, and visible to diagnostics/budget surfaces

## Memory laws
- browser and outliner projections are index-backed and partial where possible
- diagnostics and collaboration views are paged or windowed when unbounded histories exist
- preview artifacts are discardable
