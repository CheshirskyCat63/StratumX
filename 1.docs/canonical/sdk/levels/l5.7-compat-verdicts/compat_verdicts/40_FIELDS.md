# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| compat_verdict_id | CompatVerdictId | required | stable identity of one compatibility verdict | must remain unique for one evaluated tuple |
| evaluated_version_set_id | CompatVersionSetId | required | identifies the evaluated version set | must resolve through `compat_versions` |
| evaluated_capability_set_id | CompatCapabilitySetId | required | identifies the evaluated capability set | must resolve through `compat_capabilities` |
| evaluated_profile_id | CompatProfileId | required | identifies the evaluated profile | must resolve through `compat_profiles` |
| verdict_status | CompatVerdictStatus | required | declares pass, degraded-pass, or fail | must use the shared registry verdict enum |
| blocking_reason_refs | SmallVec<CompatBlockingReasonRef> | optional | points to structured reasons for a degraded or failed verdict | reasons only; no embedded plan or UI text |
