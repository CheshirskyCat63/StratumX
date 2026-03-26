# L6.0 Authority Core - Invalidation

## Purpose

This document defines the invalidation model for L6.0 authority core.

## Invalidation Rules

### Session Invalidation
- **Trigger**: Session timeout, explicit close, or error condition
- **Effect**: All session state is invalidated
- **Propagation**: Invalidation propagates to all dependent transactions

### Command Invalidation
- **Trigger**: Command timeout, validation failure, or cancellation
- **Effect**: Command is marked invalid and removed from routing queue
- **Propagation**: Invalidation does not affect other commands in session

### Transaction Invalidation
- **Trigger**: Transaction failure, rollback, or session invalidation
- **Effect**: Transaction is marked invalid in L6.2 ledger
- **Propagation**: Invalidation propagates to all dependent snapshots

## Invalidation Verification

All invalidation rules are verified through:
- Invalidation tests
- Propagation tests
- Timeout tests
- Error condition tests
