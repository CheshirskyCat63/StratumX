# L6.0 Authority Core - Threading

## Purpose

This document defines the threading model for L6.0 authority core.

## Threading Model

### Thread Safety
- **Authority Sessions**: Each session is `Send + Sync`
- **Command Routing**: Command routing is thread-safe
- **Transaction Initiation**: Transaction initiation is thread-safe

### Concurrency Rules
1. **Session Isolation**: Each session runs in isolation, no cross-session interference
2. **Command Serialization**: Commands within a session are serialized
3. **Transaction Ordering**: Transactions are ordered by session and timestamp
4. **Lock-Free Queries**: Authority state queries are lock-free

## Threading Verification

All threading properties are verified through:
- Thread safety tests
- Concurrency stress tests
- Session isolation tests
- Lock-free query tests
