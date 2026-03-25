# Memory, GPU, Disk Discipline

## Memory
- no unbounded caches
- no hidden duplicate stores
- no whole-project raw dump by default
- bounded evidence packs only
- equal data classes must co-reside when they share locality
- bulk authoring data should prefer snapshots, indices, streams, and artifacts over mutable heaps

## GPU
- no silent long-lived preview residency
- preview resources must be explicit, budgeted, and evictable
- workspace/UI resources and preview resources must not collapse into one uncontrolled pool
- build and release staging must not pin editor preview residency

## Disk
- artifacts must be explicit and manifest-backed
- caches must be evictable
- campaign logs and reasoning traces must be bounded and compressible
- no silent growth directories
- reporting outputs and release packages must declare storage class and retention class
