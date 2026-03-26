# L5.0 Link Ingress Packets - Libraries

## Purpose

This document defines the library dependencies for L5.0 link ingress packets.

## Library Dependencies

| Library | Purpose | Version | Notes |
|---------|---------|---------|-------|
| `stratumx_sdk_core` | Core SDK types and traits | 1.0.0 | Required for all L5 levels |
| `stratumx_sdk_l5_packets` | Packet type definitions | 1.0.0 | This level's primary library |
| `stratumx_engine_l4_export` | L4 export surfaces | 1.0.0 | Required for L4 sync |

## Forbidden Dependencies

L5.0 must not depend on:
- Any engine internal libraries (only L4 export surfaces)
- Any L6 tooling libraries (L5 is consumed by L6, not vice versa)
- Any L5.1+ libraries (packets are lowest L5 level)

## Library Boundaries

`stratumx_sdk_l5_packets` exports:
- Packet type definitions
- Packet field accessors (read-only)
- Packet validation functions
- Packet serialization/deserialization

`stratumx_sdk_l5_packets` does not export:
- Packet mutation functions (packets are immutable after creation)
- Engine internal state
- L6 tooling interfaces
