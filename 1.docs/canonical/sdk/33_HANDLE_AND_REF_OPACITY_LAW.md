# Handle And Ref Opacity Law

Opaque handles and refs must never reveal:
- raw memory addresses
- raw ECS row layout
- engine allocator identity
- cross-layer mutable storage internals

Opaque export law exists to keep hot-path bridge data tiny and ownership-safe.
