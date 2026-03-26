# Editor Activation and Cold Surface Model

## Purpose

This document defines the editor-wide activation and cold surface model: hot/warm/cold surfaces, lazy activation, release/deactivation rules, and no hidden background ownership.

## Scope

The editor package (`L8` through `L11`) uses activation discipline to manage resource lifetime and prevent unbounded resource consumption.

## Surface Temperature Classes

### Hot Surface
- Visible and actively consuming resources
- Full budget allocation
- Immediate refresh on invalidation
- Immediate input handling

### Warm Surface
- Not visible but retains some resources for fast reactivation
- Reduced budget allocation (e.g., 25% of full budget)
- Deferred refresh on invalidation
- No input handling

### Cold Surface
- Not visible and has released all resources
- Zero budget allocation
- No refresh on invalidation (refresh on reactivation)
- No input handling

## Lazy Activation

### Activation Discipline
- Surfaces are activated on first use (lazy activation)
- Surfaces are not activated on editor startup (unless previously open)
- Surfaces are not activated on project load (unless previously open)
- Surfaces are activated on user action (open panel, switch viewport mode, activate suite)

### Activation Examples
- Outliner panel: Activated on first open, not on editor startup
- Terrain suite: Activated on first use, not on project load
- Import service: Activated on first import request, not on editor startup

## Release and Deactivation Rules

### Deactivation Discipline
- Surfaces are deactivated when closed (panel close, viewport close)
- Surfaces are deactivated when hidden (panel tab switch, viewport minimize)
- Surfaces are deactivated when idle (no user interaction for N seconds)
- Surfaces are deactivated when budget is exceeded (cold surfaces first)

### Deactivation Examples
- Outliner panel: Deactivated when closed or hidden
- Terrain suite: Deactivated when switched to another suite
- Import service: Deactivated when idle for 5 minutes

### Release Discipline
- Surfaces release resources on deactivation
- Surfaces release jobs on deactivation (cancel background jobs)
- Surfaces release caches on deactivation (if cold)
- Surfaces release UI state on deactivation (if cold)

### Release Examples
- Outliner panel: Releases UI state and cache on deactivation
- Terrain suite: Releases preview cache and background jobs on deactivation
- Import service: Releases import jobs and temporary artifacts on deactivation

## Temperature Transitions

### Hot → Warm
- Triggered by: Panel hidden, viewport minimized
- Actions: Reduce budget allocation, defer refresh, stop input handling

### Warm → Cold
- Triggered by: Idle timeout, budget pressure
- Actions: Release resources, release caches, release UI state

### Cold → Warm
- Triggered by: Panel shown, viewport restored
- Actions: Allocate budget, load caches, load UI state

### Warm → Hot
- Triggered by: Panel focused, viewport focused
- Actions: Allocate full budget, immediate refresh, start input handling

## Forbidden Hidden Background Ownership

The editor must NOT:
- Maintain hidden background jobs without user visibility
- Maintain hidden caches without resource discipline
- Maintain hidden parallel truth
- Maintain hidden mutation queues
- Maintain hidden undo/redo state

### Visibility Discipline
- All background jobs must be visible in UI (progress bar, status bar, panel)
- All caches must be bounded by budget
- All parallel state must be explicit and visible
- All mutation queues must be explicit and visible
- All undo/redo state must be explicit and visible

## Activation and Collaboration

### Collaboration Discipline
- Collaboration surfaces are activated on session start
- Collaboration surfaces are deactivated on session end
- Collaboration surfaces may remain warm during session (if hidden)
- Collaboration surfaces must not maintain hidden background state

## Activation and Performance

### Performance Discipline
- Activation must not block the UI thread
- Activation must not cause frame drops in viewport
- Activation must not cause input lag
- Activation must be cancellable

### Performance Optimization
- Activation uses background threads
- Activation uses incremental loading (load only what's needed)
- Activation uses caching (reuse previously loaded state)
- Activation uses batching (multiple activations in one batch)

## Activation Examples

### Viewport Activation
1. User opens viewport
2. Viewport transitions from cold → warm
3. Viewport allocates budget
4. Viewport loads render targets
5. Viewport loads preview cache
6. Viewport transitions from warm → hot
7. Viewport starts rendering

### Panel Activation
1. User opens panel
2. Panel transitions from cold → warm
3. Panel allocates budget
4. Panel loads UI state
5. Panel loads cache
6. Panel transitions from warm → hot
7. Panel starts input handling

### Suite Activation
1. User activates suite
2. Suite transitions from cold → warm
3. Suite allocates budget
4. Suite loads suite-specific state
5. Suite loads preview cache
6. Suite transitions from warm → hot
7. Suite starts input handling

### Service Activation
1. User triggers service operation
2. Service transitions from cold → warm
3. Service allocates budget
4. Service loads service-specific state
5. Service starts background job
6. Service transitions from warm → hot (if UI is shown)
7. Service completes operation
8. Service transitions from hot → warm (if UI is hidden)
9. Service transitions from warm → cold (if idle)

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
