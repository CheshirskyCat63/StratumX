# Editor Source Control and Conflict Model

## Purpose

This document defines the editor-wide source control and conflict model: checkout/branch/merge/review/conflict posture, editor integration limits, and collaboration vs source-control separation.

## Scope

The editor package (`L8` through `L11`) integrates with source control but does not own source control truth.

## Source Control Posture

### Integration Discipline
- The editor integrates with source control via external tools (Git, Perforce, etc.)
- The editor does not own source control truth
- The editor does not implement source control logic
- The editor consumes source control state via read-only APIs

### Integration Limits
- The editor may show source control status (modified, added, deleted, conflicted)
- The editor may trigger source control operations (checkout, commit, revert)
- The editor may show source control history (log, blame)
- The editor must NOT implement source control merge logic
- The editor must NOT implement source control conflict resolution logic

## Checkout Posture

### Checkout Discipline
- The editor may trigger checkout on file modification
- The editor may show checkout status in UI
- The editor may block modification if checkout fails
- The editor must NOT bypass source control checkout

### Checkout UI
- Show checkout status in content browser (locked, unlocked, checked out by other user)
- Show checkout dialog on modification (if file is locked)
- Show checkout error dialog (if checkout fails)

## Branch Posture

### Branch Discipline
- The editor may show current branch in UI
- The editor may trigger branch switch via external tool
- The editor must reload project on branch switch
- The editor must NOT implement branch logic

### Branch UI
- Show current branch in status bar
- Show branch switch dialog (if triggered from editor)
- Show branch switch warning (unsaved changes will be lost)

## Merge Posture

### Merge Discipline
- The editor may trigger merge via external tool
- The editor must reload project on merge
- The editor must NOT implement merge logic
- The editor must NOT implement conflict resolution logic

### Merge UI
- Show merge dialog (if triggered from editor)
- Show merge warning (unsaved changes will be lost)
- Show merge result (success, conflicts detected)

## Review Posture

### Review Discipline
- The editor may show review annotations in UI
- The editor may trigger review via external tool
- The editor must NOT implement review logic
- The editor must NOT own review truth

### Review UI
- Show review annotations in viewport (if applicable)
- Show review annotations in content browser (if applicable)
- Show review panel (if applicable)

## Conflict Posture

### Conflict Discipline
- The editor may detect conflicts on file modification
- The editor may show conflict status in UI
- The editor may trigger conflict resolution via external tool
- The editor must NOT implement conflict resolution logic

### Conflict UI
- Show conflict status in content browser (conflicted)
- Show conflict dialog on modification (file is conflicted, resolve before modifying)
- Show conflict resolution dialog (if triggered from editor, launches external tool)

## Collaboration vs Source Control Separation

### Separation Discipline
- Collaboration is real-time multi-user editing
- Source control is asynchronous version control
- Collaboration and source control are separate concerns
- Collaboration does not replace source control
- Source control does not replace collaboration

### Separation Examples
- Collaboration session: Multiple users edit the same world in real-time
- Source control commit: User commits changes to repository after collaboration session
- Collaboration conflict: Two users modify the same entity in real-time (resolved immediately)
- Source control conflict: Two users commit conflicting changes (resolved via external tool)

## Source Control and Autosave

### Autosave Discipline
- Autosave does not trigger source control operations
- Autosave does not modify source-controlled files (uses temporary location)
- Autosave does not conflict with source control

## Source Control and Recovery

### Recovery Discipline
- Recovery does not trigger source control operations
- Recovery does not modify source-controlled files (uses temporary location)
- Recovery does not conflict with source control

## Source Control Integration Examples

### Git Integration
- Show Git status in content browser (modified, staged, committed)
- Trigger Git operations via external tool (commit, push, pull, merge)
- Show Git history in panel (log, blame)

### Perforce Integration
- Show Perforce status in content browser (checked out, locked, submitted)
- Trigger Perforce operations via external tool (checkout, submit, revert)
- Show Perforce history in panel (log, annotate)

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
