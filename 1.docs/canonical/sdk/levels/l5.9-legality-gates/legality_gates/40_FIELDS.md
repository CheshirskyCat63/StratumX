# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| legality_gate_eval_id | LegalityGateEvalId | required | stable identity of one legality evaluation | must remain unique for one evaluated target action |
| target_boundary_action_kind | BoundaryActionKind | required | declares which packet or control action is being gated | must use the shared registry enum |
| evaluated_profile_id | CompatProfileId | required | identifies the profile under which the action was evaluated | must resolve through `compat_profiles` |
| required_capability_flags | RequiredCapabilityFlags | required | declares the capabilities required for the action | must use the shared registry capability flags |
| gate_verdict | GateVerdict | required | declares pass, deny, or degrade | must use the shared registry verdict enum |
| blocking_reason_refs | SmallVec<GateBlockingReasonRef> | optional | points to structured reasons for deny or degrade | reasons only; no embedded plans or UI text |
