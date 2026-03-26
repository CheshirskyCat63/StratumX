# L6.0 Authority Core - Communication

## Purpose

This document defines the communication patterns for L6.0 authority core.

## Communication Patterns

### Ingress (from L8 Editor)
- **Command Submission**: L8 submits commands via authority core
- **Query Requests**: L8 queries authority state via authority core
- **Session Management**: L8 manages authority sessions via authority core

### Egress (to L6.2+ Data Planes)
- **Transaction Initiation**: Authority core initiates transactions in L6.2 ledger
- **Snapshot Requests**: Authority core requests snapshots from L6.3 plane
- **Index Queries**: Authority core queries indexes from L6.4 plane

## Communication Rules

1. **Command Validation**: All commands are validated before routing to data planes
2. **Session Isolation**: Each authority session is isolated from others
3. **Transaction Atomicity**: Transactions are atomic and consistent
4. **Query Consistency**: Queries see consistent snapshots of authority state

## Communication Verification

All communication patterns are verified through:
- Interface contract checks
- Session isolation tests
- Transaction atomicity tests
- Query consistency tests
