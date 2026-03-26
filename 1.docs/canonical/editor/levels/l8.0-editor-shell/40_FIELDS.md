# L8.0 Editor Shell Fields

## Purpose

This document defines the owned local state for the editor shell level.

## Scope

The editor shell (`L8.0`) owns local state for:
- Shell composition
- Workspace layout
- Shell-local preferences
- Command state
- Status state

## Owned Fields

### Shell Composition State
```rust
struct ShellState {
    window: WindowHandle,
    menu_bar: MenuBarState,
    toolbar: ToolbarState,
    panel_host: PanelHostState,
    viewport_host: ViewportHostState,
    status_bar: StatusBarState,
}
```

### Workspace Layout State
```rust
struct WorkspaceLayoutState {
    current_layout: Layout,
    saved_layouts: HashMap<LayoutId, Layout>,
    layout_dirty: bool,
}

struct Layout {
    panels: Vec<PanelLayout>,
    viewports: Vec<ViewportLayout>,
}

struct PanelLayout {
    id: PanelId,
    docked: bool,
    position: DockPosition,
    size: Size,
}

struct ViewportLayout {
    id: ViewportId,
    position: Position,
    size: Size,
    mode: ViewportMode,
}
```

### Shell-Local Preferences
```rust
struct ShellPreferences {
    theme: ThemeId,
    font_size: u32,
    autosave_enabled: bool,
    autosave_interval: Duration,
}
```

### Command State
```rust
struct CommandState {
    commands: HashMap<CommandId, Command>,
    shortcuts: HashMap<Shortcut, CommandId>,
    enabled_commands: HashSet<CommandId>,
}
```

### Status State
```rust
struct StatusState {
    status_text: String,
    progress: Option<(f32, String)>,
    notifications: Vec<Notification>,
}

struct Notification {
    level: NotificationLevel,
    message: String,
    timestamp: Timestamp,
}
```

## Forbidden Fields

The editor shell must NOT own:
- World truth or entity authority
- Asset authority or compilation state
- Package state or dependency resolution
- Rendering pipelines or shader compilation
- Simulation or physics state

## Field Lifetime

### Persistent Fields
- Workspace layout state (persisted to disk)
- Shell-local preferences (persisted to disk)

### Session Fields
- Shell composition state (recreated on startup)
- Command state (recreated on startup)
- Status state (cleared on startup)

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package, level `L8.0`.
