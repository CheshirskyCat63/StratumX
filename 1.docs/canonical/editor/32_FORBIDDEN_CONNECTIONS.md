# Editor Forbidden Connections

## Purpose

This document defines explicit forbidden patterns for the editor package: reads, writes, ownership, and authority leakage.

## Scope

The editor package (`L8` through `L11`) must never bypass lower-stack authority or maintain shadow truth.

## Forbidden Reads

The editor must NOT:
- Read world truth except via `L6.0` authority-core
- Read entity authority except via `L6.0` authority-core
- Read asset authority except via `L6.0` authority-core
- Read package state except via `L6.2` package-manager
- Read compilation state except via `L6.1` asset-compiler (read-only)
- Read engine state directly (must flow through tooling)
- Read SDK state directly for world/asset/entity operations (must flow through tooling)

## Forbidden Writes

The editor must NOT:
- Write world truth directly (must use `L6.0` mutation requests)
- Write entity authority directly (must use `L6.0` mutation requests)
- Write asset authority directly (must use `L6.1` compilation requests)
- Write package state directly (must use `L6.2` package-manager requests)
- Write compilation state directly (must use `L6.1` compilation requests)
- Write engine state directly
- Write SDK state directly for world/asset/entity operations

## Forbidden Ownership

The editor must NOT:
- Own world truth or entity authority
- Own asset authority or compilation state
- Own package state or dependency resolution
- Own rendering pipelines or shader compilation
- Own simulation or physics state
- Own network transport or protocol
- Maintain shadow world or parallel entity state
- Maintain shadow asset or parallel compilation state
- Maintain shadow package or parallel dependency state

## Forbidden Authority Leakage

### Editor Shell (L8.0)
The shell must NOT:
- Own world truth
- Own asset authority
- Own package state
- Maintain hidden background state
- Bypass lower-stack authority

### Viewport System (L8.1)
The viewport must NOT:
- Own world truth (consumes via `L6.0`)
- Own rendering pipelines (consumes via tooling preview runtime)
- Maintain shadow world state
- Bypass lower-stack authority

### Panel Systems (L8.2-L8.5)
Panels must NOT:
- Own world truth (outliner consumes via `L6.0`)
- Own asset authority (content browser consumes via `L6.0`)
- Own package state (content browser consumes via `L6.2`)
- Maintain shadow truth
- Bypass lower-stack authority

### Interaction Routing (L8.8)
Interaction routing must NOT:
- Own world truth
- Own selection state (coordinates selection, does not own truth)
- Maintain hidden state
- Bypass lower-stack authority

### Assistant Surface (L8.9)
The assistant surface must NOT:
- Own world truth
- Own asset authority
- Own package state
- Execute commands without user approval
- Maintain hidden background state
- Bypass lower-stack authority

### Build/Release Surface (L8.11)
The build/release surface must NOT:
- Own build state (consumes via `L7.0` build-orchestrator)
- Own release state (consumes via `L7.1` release-packager)
- Maintain shadow build state
- Bypass lower-stack authority

### Domain Suites (L9.0-L9.3)
Suites must NOT:
- Own world truth (consume via `L6.0`)
- Own asset authority (consume via `L6.0`)
- Maintain shadow truth
- Bypass lower-stack authority
- Become project-specific (suites are universal)

### Services (L10.0-L10.7)
Services must NOT:
- Own world truth
- Own asset authority
- Own package state
- Maintain shadow truth
- Bypass lower-stack authority
- Execute without user approval (except automation with explicit user configuration)

### Plugin/Extension Host (L10.5)
The plugin host must NOT:
- Grant plugins direct access to world truth
- Grant plugins direct access to asset authority
- Grant plugins direct access to package state
- Allow plugins to bypass capability/trust/sandbox law
- Allow plugins to maintain hidden background state

### Collaboration Surfaces (L11.0-L11.5)
Collaboration surfaces must NOT:
- Own world truth (consume via `L6.0`)
- Own asset authority (consume via `L6.0`)
- Become source of truth for review/annotation/playtest
- Maintain shadow truth
- Bypass lower-stack authority

## Forbidden Hidden State

The editor must NOT:
- Maintain hidden background jobs without user visibility
- Maintain hidden caches without resource discipline
- Maintain hidden parallel truth
- Maintain hidden mutation queues
- Maintain hidden undo/redo state (undo/redo must be explicit and visible)

## Forbidden Bypass Patterns

The editor must NOT:
- Bypass `L6.0` authority-core to reach engine directly
- Bypass `L6.1` asset-compiler to reach engine directly
- Bypass `L6.2` package-manager to reach file system directly
- Bypass `L7.0` build-orchestrator to reach build tools directly
- Bypass `L7.1` release-packager to reach packaging tools directly
- Bypass `L6A.0` assistant-runtime to reach planning-brain directly

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
