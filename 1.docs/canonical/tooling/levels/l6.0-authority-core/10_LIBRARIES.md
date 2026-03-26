# L6.0 Authority Core - Libraries

## Purpose

This document defines the library dependencies for L6.0 authority core.

## Library Dependencies

| Library | Purpose | Version | Notes |
|---------|---------|---------|-------|
| `stratumx_tooling_core` | Core tooling types and traits | 1.0.0 | Required for all L6 levels |
| `stratumx_tooling_l6_authority` | Authority core implementation | 1.0.0 | This level's primary library |
| `stratumx_sdk_l5` | SDK L5 bridge types | 1.0.0 | Required for L5 consumption |

## Forbidden Dependencies

L6.0 must not depend on:
- Any engine internal libraries (only SDK L5 bridge)
- Any L8+ editor libraries (L6 is consumed by L8, not vice versa)
- Any L6.1+ data plane libraries (authority core is foundational)

## Library Boundaries

`stratumx_tooling_l6_authority` exports:
- Authority session management
- Command routing and validation
- Transaction initiation
- Authority state queries

`stratumx_tooling_l6_authority` does not export:
- Transaction ledger internals (owned by L6.2)
- Snapshot plane internals (owned by L6.3)
- Editor UI state (owned by L8+)
