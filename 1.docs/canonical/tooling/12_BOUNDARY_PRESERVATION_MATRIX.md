# Boundary Preservation Matrix

| Boundary | Must preserve | Must not leak |
|---|---|---|
| `L5 -> L6` | bridge facts, handles, refs, artifact refs, verdict tables, snapshots, batches, and ingress publication only | engine internals, editor-shaped structs |
| `L6 -> L6A` | bounded runtime data, evidence inputs, proposal surfaces, apply/revert results | authority ownership, hidden mutable truth |
| `L6A -> L7A` | bounded goals, bounded context, evidence requests, plan requests, routing requests | raw editor authority state |
| `L7 -> L6` | compiled campaign bundles, task bundles, governance policies, automation requests | frame-level runtime authority |
| `L7A -> L6A` | plan bundles, proposal intents, canon constraints, optimization alternatives, migration plans | direct apply ownership |
| any upper layer -> engine | forbidden around `L5` | all direct engine mutation |
