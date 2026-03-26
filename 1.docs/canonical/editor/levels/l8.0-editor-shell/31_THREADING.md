# L8.0 Editor Shell Threading

## Purpose

This document defines the threading model for the editor shell level.

## Scope

The editor shell (`L8.0`) uses structured threading for:
- UI thread (shell composition, menu, toolbar, status bar)
- Background jobs (layout persistence, workspace migration)

## UI Thread

### UI Thread Responsibilities
- Shell composition and layout
- Menu bar and toolbar rendering
- Panel and viewport hosting
- Status bar updates
- Command dispatch
- Input routing

### UI Thread Discipline
- All UI operations must run on the UI thread
- No blocking operations on the UI thread
- No long-running operations on the UI thread
- No direct file I/O on the UI thread (use background jobs)

## Background Jobs

### Layout Persistence Jobs
- Layout persistence requests are dispatched to background threads
- Layout persistence results are posted back to the UI thread
- Layout persistence jobs may be cancelled

### Workspace Migration Jobs
- Workspace migration requests are dispatched to background threads
- Workspace migration results are posted back to the UI thread
- Workspace migration jobs may be cancelled
- Workspace migration jobs may report progress

## Cancellation Rules

### Cancellation Discipline
- All background jobs must accept a cancellation token
- All background jobs must check cancellation periodically
- All background jobs must clean up resources on cancellation

### Cancellation Triggers
- User cancellation (cancel button, close shell)
- Superseding request (new layout persistence supersedes old)
- Timeout (long-running job exceeds timeout)

## Ownership Rules

### Job Ownership
- Jobs are owned by the shell
- Jobs are cancelled when the shell is closed

### Resource Ownership
- Resources are owned by the shell
- Resources are released when the shell is closed

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package, level `L8.0`.
