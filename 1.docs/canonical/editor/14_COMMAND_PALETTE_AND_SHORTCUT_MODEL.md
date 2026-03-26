# Command Palette and Shortcut Model

## Palette responsibilities
- command discovery
- fuzzy search of actions, views, tools, assets, and documentation surfaces
- context-sensitive action ranking
- safe handoff to lower-stack requests

## Shortcut responsibilities
- global shortcuts
- workspace-scoped shortcuts
- suite-scoped shortcuts
- context-sensitive temporary bindings

## Laws
- shortcuts never bypass legality and authority laws
- palette invocation may stage commands but not commit hidden changes
