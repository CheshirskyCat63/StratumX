# Editor Undo/Redo and History Model

## Purpose

This document defines the editor-wide undo/redo model: command journal posture, reversible operation classes, history segmentation, cross-surface undo safety, and forbidden hidden mutation.

## Scope

The editor package (`L8` through `L11`) provides undo/redo for all user-visible mutations.

## Command Journal Posture

### Journal Structure
```rust
struct CommandJournal {
    history: Vec<Command>,
    current: usize,
    max_size: usize,
}

struct Command {
    id: CommandId,
    label: String,
    execute: Box<dyn Fn() -> Result<(), Error>>,
    undo: Box<dyn Fn() -> Result<(), Error>>,
    redo: Box<dyn Fn() -> Result<(), Error>>,
}
```

### Journal Discipline
- All user-visible mutations must be journaled
- All journaled commands must be reversible
- All journaled commands must have a user-visible label
- All journaled commands must clean up resources on undo

## Reversible Operation Classes

### World Mutations
- Entity creation/deletion
- Component addition/removal
- Property modification
- Hierarchy changes (parent/child)
- Transform changes (position, rotation, scale)

### Asset Mutations
- Asset creation/deletion
- Asset import/export
- Asset property modification
- Asset dependency changes

### Layout Mutations
- Panel open/close
- Panel dock/undock
- Panel resize
- Viewport mode change
- Workspace layout change

### Selection Mutations
- Selection change (if user-initiated)
- Focus change (if user-initiated)

### Tool Mutations
- Tool activation
- Mode change
- Gizmo manipulation

## History Segmentation

### Segmentation Rules
- History is segmented by user action
- History is segmented by time (e.g., 1 second of inactivity)
- History is segmented by surface (e.g., viewport vs panel)
- History is segmented by domain (e.g., world vs asset)

### Segmentation Examples
- Multiple property changes in rapid succession are grouped into a single undo step
- Multiple entity selections in rapid succession are grouped into a single undo step
- Multiple gizmo manipulations in rapid succession are grouped into a single undo step

## Cross-Surface Undo Safety

### Cross-Surface Discipline
- Undo/redo must be safe across surfaces
- Undo/redo must not leave surfaces in inconsistent state
- Undo/redo must invalidate affected surfaces
- Undo/redo must refresh affected surfaces

### Cross-Surface Examples
- Undoing entity deletion in viewport must refresh outliner
- Undoing asset import in content browser must refresh inspector
- Undoing layout change in shell must refresh all panels

## Forbidden Hidden Mutation

The editor must NOT:
- Mutate world truth without journaling
- Mutate asset authority without journaling
- Mutate layout without journaling
- Mutate selection without journaling (if user-initiated)
- Maintain hidden undo/redo state
- Bypass command journal for user-visible mutations

## Undo/Redo Limits

### Memory Limits
- Command journal has a maximum size (e.g., 1000 commands)
- Oldest commands are evicted when journal is full
- Large commands (e.g., bulk import) may be evicted sooner

### Time Limits
- Command journal may have a time limit (e.g., 1 hour)
- Commands older than time limit are evicted

### Persistence Limits
- Command journal is not persisted across editor sessions
- Command journal is cleared on project close
- Command journal is cleared on workspace reset

## Undo/Redo UI

### UI Elements
- Undo button (toolbar, menu, shortcut)
- Redo button (toolbar, menu, shortcut)
- Undo history panel (optional, shows command labels)

### UI Discipline
- Undo/redo buttons are enabled/disabled based on journal state
- Undo/redo buttons show tooltip with command label
- Undo history panel shows command labels and timestamps

## Undo/Redo and Collaboration

### Collaboration Discipline
- Undo/redo is local to each user in a collaboration session
- Undo/redo does not affect other users' changes
- Undo/redo may conflict with other users' changes (conflict resolution required)

### Conflict Resolution
- Conflicts are detected on undo/redo
- Conflicts are presented to the user
- User may choose to resolve conflict or cancel undo/redo

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
