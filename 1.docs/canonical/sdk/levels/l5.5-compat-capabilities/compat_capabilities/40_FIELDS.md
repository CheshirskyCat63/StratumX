# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| compat_capability_set_id | CompatCapabilitySetId | required | stable identity of one capability set | must remain stable for the declared capability tuple |
| engine_capability_flags | EngineCapabilityFlags | required | declares public L4 capabilities seen by the bridge | must use the shared registry capability flags |
| bridge_capability_flags | BridgeCapabilityFlags | required | declares capabilities that L5 itself provides | must use the shared registry capability flags |
| unsupported_feature_flags | UnsupportedFeatureFlags | optional | declares known blocked feature families | must be descriptive only |
| publication_epoch | PublicationEpoch | required | monotonic publication epoch for capability truth | must increase on capability-truth changes |
