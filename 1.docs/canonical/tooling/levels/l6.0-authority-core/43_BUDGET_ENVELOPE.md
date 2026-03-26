# L6.0 Authority Core - Budget Envelope

## Purpose

This document defines the budget envelope for L6.0 authority core hot paths.

## Latency Posture

| Operation | Target Latency | Maximum Latency | Notes |
|-----------|----------------|-----------------|-------|
| Session Create | < 1ms | 5ms | Hot path, must be fast |
| Command Submit | < 100μs | 1ms | Hot path, must be fast |
| Command Validate | < 500μs | 2ms | Hot path, must be fast |
| Transaction Initiate | < 1ms | 5ms | Hot path, must be fast |
| Session Query | < 100μs | 1ms | Hot path, must be fast |

## Allocation Posture

| Operation | Allocation Budget | Notes |
|-----------|-------------------|-------|
| Session Create | < 1KB | Minimal allocation |
| Command Submit | 0 bytes | Zero allocation on hot path |
| Command Validate | 0 bytes | Zero allocation on hot path |
| Transaction Initiate | < 512 bytes | Minimal allocation |
| Session Query | 0 bytes | Zero allocation on hot path |

## Refresh/Invalidation Posture

- **Session Refresh**: Every 60 seconds or on explicit refresh
- **Command Invalidation**: Immediate on timeout or cancellation
- **Transaction Invalidation**: Immediate on failure or rollback

## Cancellation Posture

- **Session Cancellation**: Graceful shutdown with cleanup
- **Command Cancellation**: Immediate removal from routing queue
- **Transaction Cancellation**: Rollback with cleanup

## Hot Path Restrictions

1. **No Blocking I/O**: All hot path operations are non-blocking
2. **No Heap Allocation**: Command submit/validate are zero-allocation
3. **No Lock Contention**: All hot path operations use lock-free data structures
4. **No Unbounded Loops**: All hot path operations have bounded iteration

## Prohibited Budget Leaks

1. **Session Leak**: Sessions must be explicitly closed or timeout
2. **Command Leak**: Commands must be explicitly completed or cancelled
3. **Transaction Leak**: Transactions must be explicitly committed or rolled back
4. **Memory Leak**: All allocations must be freed on operation completion
