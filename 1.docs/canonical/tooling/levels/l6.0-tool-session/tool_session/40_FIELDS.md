        # Fields

        - `tool_session_id: SessionId` required, unique owner identity.
- `project_id: ProjectId` required, immutable for the session lifetime.
- `engine_session_handle_ref: EngineSessionHandleRef` required, owned below L6.
- `active_profile_ref: ProfileRef` required, typed ref only.
- `session_state: SessionState` required, closed enum.

        Cardinality, ownership, and invariants are part of canon.
