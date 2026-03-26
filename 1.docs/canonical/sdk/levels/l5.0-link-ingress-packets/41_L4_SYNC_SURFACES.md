# L5.0 Link Ingress Packets - L4 Sync Surfaces

## Purpose

This document defines the L4 synchronization surfaces for L5.0 link ingress packets.

## L4 Sync Surfaces

### Packet Creation Surface
- **Interface**: `L4PacketCreator`
- **Purpose**: L4 creates packets via this surface
- **Operations**: `create_packet(payload) -> PacketId`
- **Synchronization**: Synchronous creation, asynchronous delivery

### Packet Batch Surface
- **Interface**: `L4PacketBatch`
- **Purpose**: L4 delivers packets in batches
- **Operations**: `deliver_batch(packets: Vec<Packet>)`
- **Synchronization**: Batch delivery for efficiency

### Packet Snapshot Surface
- **Interface**: `L4PacketSnapshot`
- **Purpose**: L4 syncs packet state via snapshots
- **Operations**: `sync_snapshot(epoch: EpochMarker, packets: Vec<Packet>)`
- **Synchronization**: Snapshot-isolated delivery

### Packet Invalidation Surface
- **Interface**: `L4PacketInvalidation`
- **Purpose**: L4 signals packet invalidation
- **Operations**: `invalidate_packets(epoch: EpochMarker, packet_ids: Vec<PacketId>)`
- **Synchronization**: Epoch-based invalidation

## Sync Rules

1. **Snapshot Isolation**: All packet reads are snapshot-isolated
2. **Batch Delivery**: Packets are delivered in batches for efficiency
3. **Epoch Consistency**: All packets in a batch have the same epoch
4. **Invalidation Ordering**: Invalidation signals are ordered by epoch

## Sync Verification

All sync surfaces are verified through:
- Sync surface tests
- Snapshot isolation tests
- Batch delivery tests
- Epoch consistency tests
