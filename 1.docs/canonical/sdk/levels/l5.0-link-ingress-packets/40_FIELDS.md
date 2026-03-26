# L5.0 Link Ingress Packets - Fields

## Purpose

This document defines the field invariants for L5.0 link ingress packets.

## Field Invariants

### Packet Identity
- **Field**: `packet_id: PacketId`
- **Invariant**: Unique per packet, immutable after creation
- **Verification**: Identity uniqueness tests

### Packet Timestamp
- **Field**: `timestamp: Timestamp`
- **Invariant**: Monotonically increasing within a session, immutable after creation
- **Verification**: Timestamp ordering tests

### Packet Payload
- **Field**: `payload: PacketPayload`
- **Invariant**: Immutable after creation, validated on creation
- **Verification**: Payload validation tests

### Packet Epoch
- **Field**: `epoch: EpochMarker`
- **Invariant**: Matches L4 snapshot epoch, immutable after creation
- **Verification**: Epoch consistency tests

## No Hidden Store

All packet state is externalized:
- No internal hidden state
- All fields accessible via public accessors
- All state transitions governed by documented rules

## Field Verification

All field invariants are verified through:
- Field invariant tests
- Immutability enforcement
- Validation tests
- State transition tests
