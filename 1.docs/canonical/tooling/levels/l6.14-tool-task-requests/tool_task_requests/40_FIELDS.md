        # Fields

        - `task_request_id: TaskRequestId` required.
- `tool_id: ToolId` required.
- `task_kind: TaskKind` required.
- `task_input_refs: TypedRef[]` required, many.
- `priority: Priority` required.
- `cancellation_ref: CancellationRef` optional.

        Cardinality, ownership, and invariants are part of canon.
