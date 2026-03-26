# L8.0 Editor Shell Libraries

## Purpose

This document defines the library classes owned by the editor shell level.

## Scope

The editor shell (`L8.0`) owns library classes for:
- Shell composition and window management
- Menu bar and toolbar
- Workspace layout persistence
- Command dispatch and routing
- Status bar and notifications

## Owned Library Classes

### Shell Composition
```rust
struct EditorShell {
    window: WindowHandle,
    menu_bar: MenuBar,
    toolbar: Toolbar,
    panel_host: PanelHost,
    viewport_host: ViewportHost,
    status_bar: StatusBar,
    layout_manager: LayoutManager,
}
```

### Menu Bar
```rust
struct MenuBar {
    menus: Vec<Menu>,
}

struct Menu {
    label: String,
    items: Vec<MenuItem>,
}

enum MenuItem {
    Command(CommandId),
    Submenu(Menu),
    Separator,
}
```

### Toolbar
```rust
struct Toolbar {
    buttons: Vec<ToolbarButton>,
}

struct ToolbarButton {
    icon: Icon,
    command: CommandId,
    tooltip: String,
    enabled: bool,
}
```

### Panel Host
```rust
struct PanelHost {
    panels: HashMap<PanelId, Panel>,
    layout: PanelLayout,
}

struct Panel {
    id: PanelId,
    title: String,
    content: Box<dyn PanelContent>,
    docked: bool,
    position: DockPosition,
}
```

### Viewport Host
```rust
struct ViewportHost {
    viewports: HashMap<ViewportId, Viewport>,
    active_viewport: Option<ViewportId>,
}
```

### Status Bar
```rust
struct StatusBar {
    items: Vec<StatusBarItem>,
}

enum StatusBarItem {
    Text(String),
    Progress(f32),
    Button(CommandId),
}
```

### Layout Manager
```rust
struct LayoutManager {
    current_layout: Layout,
    saved_layouts: HashMap<LayoutId, Layout>,
}

struct Layout {
    panels: Vec<PanelLayout>,
    viewports: Vec<ViewportLayout>,
}
```

### Command Dispatcher
```rust
struct CommandDispatcher {
    commands: HashMap<CommandId, Command>,
    shortcuts: HashMap<Shortcut, CommandId>,
}
```

## Forbidden Library Classes

The editor shell must NOT own:
- World truth or entity authority
- Asset authority or compilation state
- Package state or dependency resolution
- Rendering pipelines or shader compilation
- Simulation or physics state

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package, level `L8.0`.
