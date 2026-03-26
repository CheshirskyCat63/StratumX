# L6.0 Authority Core - Fields

## Purpose

This document defines the field invariants for L6.0 authority core.

## Field Invariants

### Session Identity
- **Field**: `session_id: SessionId`
- **Invariant**: Unique per session, immutable after creation
- **Verification**: Session uniqueness tests

### Session Timestamp
- **Field**: `created_at: Timestamp`
- **Invariant**: Monotonically increasing, immutable after creation
- **Verification**: Timestamp ordering tests

### Session Authority
- **Field**: `authority_level: AuthorityLevel`
- **Invariant**: Immutable after creation, validated on creation
- **Verification**: Authority validation tests

### Session State
- **Field**: `state: SessionState`
- **Invariant**: State transitions follow documented state machine
- **Verification**: State transition tests

## No Hidden Store

All authority core state is externalized:
- No internal hidden state
- All fields accessible via public accessors
- All state transitions governed by documented rules

## Field Verification

All field invariants are verified through:
- Field invariant tests
- State machine verification
- Validation tests
- State transition tests
