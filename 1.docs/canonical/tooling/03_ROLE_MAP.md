# Role Map

## Owner facts
- `tool_session`
- `tool_selection`
- `tool_focus_refs`
- `tool_activation_state`

## Rules
- `tool_activation_rules`

## Requests and intents
- `tool_preview_requests`
- `tool_content_intents`
- `tool_scene_intents`
- `tool_release_intents`
- `tool_assistant_intents`
- `tool_task_requests`

## Events and observations
- `tool_diagnostics_events`

## Results and projections
- `tool_preview_results`
- `tool_diagnostics_views`
- `tool_inspection_views`
- `tool_task_results`

## UI attachment refs
- `tool_panel_refs`
- `tool_view_refs`

## Family composition
- all `L6.F*` families

## Hard distinctions
- facts own compact state;
- rules declare activation conditions only;
- requests and intents ask for work but do not perform work;
- events publish observations only;
- projections render read-side structure only;
- result refs surface completed work only;
- families compose lanes but own no new truth.
