# Scope

This package defines the editor as a **complete product** and not only as an internal runtime stack.

## Included
- shell composition
- viewport and navigation
- overlays, gizmos, and mode bars
- content browser, outliner, world browser, details/inspector
- command palette, shortcuts, workspaces, docking
- play, simulate, debug, profiling, diagnostics, build, release
- assistant dock and assistant proposal surfaces
- domain authoring suites
- extensibility and production operations

## Excluded
- lower-stack truth authority
- raw engine runtime implementation
- final game-specific content schemas

## Product law
The editor must be able to author a full game project from inside the editor while keeping lower-stack authority clean and bounded.
