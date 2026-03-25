# Engine L4 Binding Map

| Engine export surface | L5 module | Surface form | Notes |
|---|---|---|---|
| packet sink | `link_ingress_packets` | ordered ingress sink | canonical write packet lane; payload body ownership stays outside the sink header |
| control sink | `link_ingress_controls` | ordered ingress sink | canonical control lane; execution signal semantics remain typed |
| observation batch source | `link_egress_observations` | immutable egress batch | canonical read event lane; fanout-safe publication only |
| metric batch source | `link_egress_metrics` | immutable egress batch | canonical read metric lane; fanout-safe publication only |
| version fact table | `compat_versions` | immutable bridge snapshot | immutable version tuples only |
| capability fact table | `compat_capabilities` | immutable bridge snapshot | immutable capability tuples only |
| profile fact table | `compat_profiles` | immutable bridge snapshot | immutable cost/profile tuples only |
| session handle table | `engine_session_handles` | immutable bridge snapshot | no editor semantics |
| object handle table | `engine_object_handles` | immutable bridge snapshot | no editor semantics |
| runtime handle table | `engine_runtime_handles` | immutable bridge snapshot | no editor semantics |
| identity projection table | `engine_identity_refs` | immutable bridge snapshot | read-side only |
| state projection table | `engine_state_refs` | immutable bridge snapshot | read-side only |
| artifact projection table | `engine_artifact_refs` | immutable bridge snapshot | generated-product only |
| bridge epoch signal | package-wide bind | monotonic invalidation signal | drives snapshot swap and cursor continuity rules |
