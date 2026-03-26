# L8.0 Editor Shell Invalidation and Refresh

## Purpose

This document defines the invalidation and refresh model for the editor shell level.

## Scope

The editor shell (`L8.0`) uses invalidation and refresh for:
- Menu bar updates (command enabled/disabled)
- Toolbar updates (button enabled/disabled)
- Status bar updates (progress, notifications)
- Layout updates (panel dock/undock, viewport open/close)

## Invalidation Signals

### Invalidation Structure
```rust
enum ShellInvalidationSignal {
    CommandStateChanged(Vec<CommandId>),
    LayoutChanged,
    StatusChanged,
    PreferencesChanged,
}
```

### Invalidation Sources
- Command state changes (command enabled/disabled)
- Layout changes (panel dock/undock, viewport open/close)
- Status changes (progress, notifications)
- Preferences changes (theme, font size)

## Refresh Rules

### Refresh Discipline
- Refresh is triggered by invalidation
- Refresh is batched (multiple invalidations in one refresh)
- Refresh is deferred if shell is not visible
- Refresh is cancelled if shell is closed

### Refresh Examples
- Command state change: Refresh menu bar and toolbar
- Layout change: Refresh panel host and viewport host
- Status change: Refresh status bar
- Preferences change: Refresh entire shell (theme, font size)

## Refresh Budget

### Budget Discipline
- Refresh must not block the UI thread
- Refresh must not cause frame drops
- Refresh must not cause input lag

### Budget Limits
- Menu bar refresh: < 1 ms
- Toolbar refresh: < 1 ms
- Status bar refresh: < 1 ms
- Layout refresh: < 10 ms

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package, level `L8.0`.
