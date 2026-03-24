        # Fields

        - `tool_id: ToolId` required.
- `activation_state: ActivationState` required, closed enum.
- `last_activated_at: UtcMillis` optional.
- `activation_reason_ref: ActivationReasonRef` optional.
- `active_task_refs: TaskRequestRef[]` optional, many.

        Cardinality, ownership, and invariants are part of canon.
