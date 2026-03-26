# L8.0 Editor Shell Budget Envelope

## Purpose

This document defines the resource budget envelope for the editor shell level.

## Scope

The editor shell (`L8.0`) enforces resource budgets for:
- Memory (shell composition, layout state, preferences)
- CPU (UI updates, layout persistence)
- Disk (layout persistence, preferences persistence)

## Memory Budget

### Budget Structure
```rust
struct ShellMemoryBudget {
    shell_composition: usize,  // e.g., 16 MB
    layout_state: usize,       // e.g., 4 MB
    preferences: usize,        // e.g., 1 MB
    command_state: usize,      // e.g., 2 MB
    status_state: usize,       // e.g., 1 MB
}
```

### Budget Discipline
- Shell composition: 16 MB (window, menu, toolbar, panel host, viewport host, status bar)
- Layout state: 4 MB (current layout, saved layouts)
- Preferences: 1 MB (theme, font size, autosave settings)
- Command state: 2 MB (commands, shortcuts, enabled commands)
- Status state: 1 MB (status text, progress, notifications)

Total: 24 MB

## CPU Budget

### Budget Structure
```rust
struct ShellCpuBudget {
    ui_update_time: Duration,  // e.g., 16 ms (60 FPS)
    refresh_time: Duration,    // e.g., 10 ms
}
```

### Budget Discipline
- UI update time: 16 ms (60 FPS)
- Refresh time: 10 ms (menu bar, toolbar, status bar, layout)

## Disk Budget

### Budget Structure
```rust
struct ShellDiskBudget {
    layout_persistence: usize,     // e.g., 10 MB
    preferences_persistence: usize, // e.g., 1 MB
}
```

### Budget Discipline
- Layout persistence: 10 MB (current layout, saved layouts)
- Preferences persistence: 1 MB (theme, font size, autosave settings)

Total: 11 MB

## Budget Enforcement

### Enforcement Discipline
- If budget is exceeded, show warning dialog
- If budget is exceeded, defer refresh until budget is available
- If budget is exceeded, reduce quality (e.g., fewer saved layouts)

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package, level `L8.0`.
