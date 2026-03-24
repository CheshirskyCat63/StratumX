# Acceptance Matrix

## Pass rule
The package is sealed only if every row with `Blocking severity = blocking` is evaluated as `pass` and has recorded evidence.

| Assertion ID | Source doc | Scope | Required assertion | Verification method | Audit owner | Blocking severity | Current status | Recorded evidence |
|---|---|---|---|---|---|---|---|---|
| ROOT-001 | 00_INDEX.md | root | index scope and ownership rules match the rewritten thin-bridge package | package audit | sdk-canon-root | blocking | pass | root_traceability_log_v12.md#EVID-ROOT-001 |
| ROOT-011 | 11_PACKAGE_LAYOUT.md | root | package layout matches the real package exactly | layout audit | sdk-canon-root | blocking | pass | root_traceability_log_v12.md#EVID-ROOT-011 |
| ROOT-026 | 26_SHARED_TYPE_REGISTRY.md | root | every shared id, enum, flag, handle, ref, verdict, and status used by the package appears in the registry | registry audit | sdk-canon-root | blocking | pass | root_traceability_log_v12.md#EVID-ROOT-026 |
| ROOT-029 | 29_DOCUMENT_AUTHORITY_ORDER.md | root | authority-order rules resolve root/constitution/layer conflicts without ambiguity | authority audit | sdk-canon-root | blocking | pass | root_traceability_log_v12.md#EVID-ROOT-029 |
| ROOT-099 | 99_AUDIT_READINESS_MATRIX.md | root | readiness matrix reports package status honestly and does not self-seal without evidence | readiness audit | sdk-canon-root | blocking | pass | root_traceability_log_v12.md#EVID-ROOT-099 |
| L5.0-001 | link_ingress_packets layer docs | layer | link_ingress_packets owns only packet boundary and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-0-001 |
| L5.1-001 | link_ingress_controls layer docs | layer | link_ingress_controls owns only control boundary and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-1-001 |
| L5.2-001 | link_egress_observations layer docs | layer | link_egress_observations owns only observation boundary and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-2-001 |
| L5.3-001 | link_egress_metrics layer docs | layer | link_egress_metrics owns only metric boundary and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-3-001 |
| L5.4-001 | compat_versions layer docs | layer | compat_versions owns only version fact and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-4-001 |
| L5.5-001 | compat_capabilities layer docs | layer | compat_capabilities owns only capability fact and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-5-001 |
| L5.6-001 | compat_profiles layer docs | layer | compat_profiles owns only profile fact and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-6-001 |
| L5.7-001 | compat_verdicts layer docs | layer | compat_verdicts owns only compatibility verdict and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-7-001 |
| L5.8-001 | transport_policies layer docs | layer | transport_policies owns only policy ref and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-8-001 |
| L5.9-001 | legality_gates layer docs | layer | legality_gates owns only legality gate and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-9-001 |
| L5.10-001 | engine_session_handles layer docs | layer | engine_session_handles owns only session handle ref and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-10-001 |
| L5.11-001 | engine_object_handles layer docs | layer | engine_object_handles owns only object handle ref and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-11-001 |
| L5.12-001 | engine_runtime_handles layer docs | layer | engine_runtime_handles owns only runtime handle ref and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-12-001 |
| L5.13-001 | engine_identity_refs layer docs | layer | engine_identity_refs owns only identity ref and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-13-001 |
| L5.14-001 | engine_state_refs layer docs | layer | engine_state_refs owns only state ref and preserves its local non-authority contract | layer audit | sdk-layer-audit | blocking | pass | layer_traceability_log_v12.md#EVID-L5-14-001 |
