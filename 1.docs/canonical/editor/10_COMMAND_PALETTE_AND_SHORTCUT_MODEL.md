# Command Palette and Shortcut Model

## Palette requirements
- fuzzy search over commands, views, tools, assets, suites, and saved actions
- recent commands
- scoped commands by active suite
- assistant-invokable command discovery
- hidden/admin/debug command separation

## Shortcut requirements
- global shortcuts
- suite-local shortcuts
- user remapping
- workspace-specific overrides
- conflict detection
- export/import keymap

## Law
Palette and shortcut layers expose only legal commands already known to the lower stack or the editor package.
They may not create secret mutation paths.
