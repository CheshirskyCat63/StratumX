# Boundary Preservation Matrix

## Absolute separations
- session != selection
- selection != focus
- diagnostics events != diagnostics views
- preview requests != preview results
- activation rules != activation state
- task requests != task results
- panel refs != view refs
- lanes != families

## Adjacent anti-collapse law
- `tool_session` may not absorb selection or diagnostics.
- `tool_selection` may not absorb inspection or scene operations.
- `tool_inspection_views` may not absorb edit authority.
- `tool_preview_requests` may not absorb result ownership.
- `tool_diagnostics_events` may not absorb view rendering.
- `tool_activation_rules` may not absorb runtime state.
- `tool_activation_state` may not absorb rule definition.
- families may not invent new owner facts outside lanes.

## Matrix

| Layer | Upward Boundary | Downward Boundary | Lateral Boundary | Status |
|-------|-----------------|-------------------|------------------|--------|
| tool_session | Defined | Defined | None | pass |
| tool_selection | Defined | Defined | None | pass |
| tool_activation_rules | Defined | Defined | None | pass |

## Rule
All L6 layer boundaries must be explicitly preserved and validated.
