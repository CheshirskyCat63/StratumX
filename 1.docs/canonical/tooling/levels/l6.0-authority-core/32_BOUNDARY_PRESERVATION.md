# L6.0 Authority Core - Boundary Preservation

## Purpose

This document defines the boundary preservation rules for L6.0 authority core.

## Boundary Rules

### L5 → L6.0 Boundary
- **Allowed**: L6.0 reads SDK L5 bridge types via immutable accessors
- **Allowed**: L6.0 queries SDK L5 state via snapshot interfaces
- **Forbidden**: L6.0 mutating SDK L5 state directly
- **Forbidden**: L6.0 accessing engine internals

### L6.0 → L6.2+ Boundary
- **Allowed**: L6.0 initiates transactions in L6.2 ledger
- **Allowed**: L6.0 requests snapshots from L6.3 plane
- **Forbidden**: L6.0 bypassing transaction ledger for state changes
- **Forbidden**: L6.0 directly mutating data plane state

### L8 → L6.0 Boundary
- **Allowed**: L8 submits commands via authority core
- **Allowed**: L8 queries authority state via authority core
- **Forbidden**: L8 bypassing authority core to access data planes
- **Forbidden**: L8 directly mutating authority state

## Boundary Verification

All boundary rules are verified through:
- Type system enforcement
- Interface contract checks
- Boundary violation tests
- Static analysis
