# Data Plane Model

Canonical `L6` planes:
- authority plane
- command plane
- transaction plane
- snapshot plane
- index plane
- derived plane
- artifact plane
- stream plane
- cache plane

Canonical `L6A` planes:
- session plane
- evidence plane
- proposal plane
- lowering plane
- safety plane
- apply/revert plane
- assistant-ui plane
- model-request plane

Canonical `L7` planes:
- campaign plane
- governance plane
- automation plane
- reporting plane

Canonical `L7A` planes:
- goal plane
- plan plane
- canon-reasoning plane
- generation plane
- optimization plane
- migration plane
- routing plane

Plane law:
- clients live around planes
- equal data classes must co-reside when they share locality, lifetime, and access pattern
- families and tools do not own secret stores when a shared plane exists
- every plane must declare storage form, mutability, rebuild policy, invalidation policy, and eviction policy when applicable

Data-class law:
- authority is mutable and minimal
- commands are typed and non-authoritative
- transactions are ordered and auditable
- snapshots are immutable and versioned
- indices are rebuildable and swap-safe
- derived views are disposable
- artifacts are deterministic and manifest-backed
- streams are bounded and forward-only
- caches are evictable and non-authoritative
- evidence packs are bounded and redactable
- plans and campaigns are compiled-control products, not runtime truth stores
