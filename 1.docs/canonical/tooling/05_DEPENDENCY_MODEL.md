# Dependency Model

## Global direction
`L6 -> L5 -> L4`

L6 never bypasses L5. L6 never imports engine internals directly.

## Core dependency law
- `tool_session` may depend on L5 handle and profile refs.
- `tool_selection` may depend on session and typed object refs only.
- `tool_focus_refs` may depend on session, selection, panel refs, and view refs.
- `tool_inspection_views` may depend on selection, focus refs, diagnostics views, and typed schema/read models from SDK.
- `tool_preview_requests` may depend on selection, focus refs, and preview policy refs surfaced by SDK.
- `tool_preview_results` may depend on preview requests and task results.
- `tool_diagnostics_events` depends only on `L5.1 link_egress`.
- `tool_diagnostics_views` depends only on diagnostics events.
- `tool_*_intents` may depend on session, selection, focus refs, and relevant SDK refs/intents.
- `tool_activation_rules` is independent of activation state.
- `tool_activation_state` may depend on activation rules and current session/focus facts.
- `tool_task_requests` may depend on intents and previews, never on engine internals.
- `tool_task_results` may depend only on task requests and result refs.

## Family dependency law
A family may compose lanes. A family may not create a new hidden authority path around lanes.
