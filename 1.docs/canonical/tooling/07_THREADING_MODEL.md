# Threading Model

## Core principle
L6 uses single-writer fact ownership and parallel derived work.

## Lane classes
- single-writer fact lanes: `tool_session`, `tool_selection`, `tool_focus_refs`, `tool_activation_state`
- read-fanout event lane: `tool_diagnostics_events`
- parallel projection lanes: `tool_inspection_views`, `tool_diagnostics_views`, `tool_preview_results`
- parallel request lanes: `tool_preview_requests`, `tool_content_intents`, `tool_scene_intents`, `tool_release_intents`, `tool_assistant_intents`, `tool_task_requests`
- result surfacing lane: `tool_task_results`

## Hard law
A lane may not smuggle in a second scheduler. L6 worker execution is request-based and disposable. Engine execution law remains below L6.
