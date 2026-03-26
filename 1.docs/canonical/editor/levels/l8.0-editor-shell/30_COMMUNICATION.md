# L8.0 Editor Shell Communication

## Purpose

This document defines the communication model for the editor shell level.

## Scope

The editor shell (`L8.0`) uses structured communication for:
- Shell commands (menu, toolbar, shortcuts)
- Panel events (open, close, dock, undock)
- Viewport events (open, close, focus, blur)
- Status updates (progress, notifications)

## Shell Commands

### Command Structure
```rust
struct ShellCommand {
    id: CommandId,
    category: CommandCategory,
    label: String,
    shortcut: Option<Shortcut>,
    enabled: bool,
}
```

### Command Categories
- `File`: New, open, save, close, exit
- `Edit`: Undo, redo, preferences
- `View`: Show/hide panels, toggle overlays
- `Window`: New window, close window, layout
- `Help`: Documentation, about

### Command Routing
- Commands are dispatched via `CommandDispatcher`
- Commands may be enabled/disabled based on shell state
- Commands may be async (e.g., "Open Project" shows file dialog)

## Panel Events

### Event Structure
```rust
enum PanelEvent {
    Opened(PanelId),
    Closed(PanelId),
    Docked(PanelId, DockPosition),
    Undocked(PanelId),
    Focused(PanelId),
    Blurred(PanelId),
}
```

### Event Routing
- Panel events are routed via `PanelHost`
- Panel events may trigger layout persistence
- Panel events may trigger resource activation/deactivation

## Viewport Events

### Event Structure
```rust
enum ViewportEvent {
    Opened(ViewportId),
    Closed(ViewportId),
    Focused(ViewportId),
    Blurred(ViewportId),
    ModeChanged(ViewportId, ViewportMode),
}
```

### Event Routing
- Viewport events are routed via `ViewportHost`
- Viewport events may trigger layout persistence
- Viewport events may trigger resource activation/deactivation

## Status Updates

### Status Structure
```rust
enum StatusUpdate {
    Progress(f32, String),
    Notification(NotificationLevel, String),
    StatusText(String),
}

enum NotificationLevel {
    Info,
    Warning,
    Error,
}
```

### Status Routing
- Status updates are routed via `StatusBar`
- Status updates may be transient (disappear after timeout)
- Status updates may be persistent (remain until dismissed)

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package, level `L8.0`.
