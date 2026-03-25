# Implementation Handoff

Implementation should preserve these crate zones:
- `tools_core_types`
- `tools_authority_tx`
- `tools_planes_read`
- `tools_artifacts_build`
- `tools_runtime_workspace`
- `tools_assistant_runtime`
- `tools_studio_orchestration`
- `tools_assistant_brain`

Mechanical guidance:
- `tools_core_types` owns ids, refs, enums, verdicts, manifests, and typed envelopes only
- `tools_authority_tx` owns commands, transactions, invalidation roots, and apply/revert bindings only
- `tools_planes_read` owns snapshots, indices, derived, streams, and caches only
- `tools_artifacts_build` owns artifacts, build, and release execution products only
- `tools_runtime_workspace` owns workspace, validation, preview, and budget runtime only
- `tools_assistant_runtime` owns sessions, evidence, proposals, lowering, safety, model IO, and assistant UI only
- `tools_studio_orchestration` owns campaigns, automation, governance, and reporting only
- `tools_assistant_brain` owns goals, plans, canon reasoning, optimization, migration, and routing only

No zone may silently take ownership that belongs to another zone.
