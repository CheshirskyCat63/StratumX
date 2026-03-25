# Family Level

Canonical family: `command_and_interaction_family`

## Composes
- palette, shortcuts, interaction routing, context menus, and drag/drop legality

## Data responsibility
- authority-facing minimal truth: command-map refs only
- snapshot classes: command routing snapshots
- index classes: shortcut and action indices
- derived classes: derived command discovery views
- artifact classes: exportable keymap artifacts
- preview classes: none required
- cache classes: command palette caches
- diagnostics classes: input and routing diagnostics
- degradation priority: `low`

This family composes editor product surfaces without owning hidden lower-stack truth.
