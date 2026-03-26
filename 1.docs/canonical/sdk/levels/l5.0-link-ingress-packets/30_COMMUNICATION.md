# L5.0 Link Ingress Packets - Communication

## Purpose

This document defines the communication patterns for L5.0 link ingress packets.

## Communication Patterns

### Ingress (from L4)
- **Packet Creation**: L4 creates packets via L4 export surfaces
- **Packet Sync**: L4 syncs packet state via snapshot/batch mechanisms
- **Packet Invalidation**: L4 signals packet invalidation via epoch markers

### Egress (to L6)
- **Packet Read**: L6 reads packet state via immutable accessors
- **Packet Query**: L6 queries packet collections via cursor interfaces
- **Packet Stream**: L6 subscribes to packet streams via stream interfaces

## Communication Rules

1. **Immutability**: Packets are immutable after creation; L6 cannot mutate packets
2. **Snapshot Isolation**: Packet reads are snapshot-isolated; L6 sees consistent packet state
3. **Batch Delivery**: Packets are delivered in batches for efficiency
4. **Cursor Pagination**: Large packet collections use cursor pagination

## Communication Verification

All communication patterns are verified through:
- Interface contract checks
- Immutability enforcement
- Snapshot isolation tests
- Batch delivery tests
