# L6.2 Transaction Ledger Communication

## Messages

```rust
struct TransactionRequest {
    command: CommandId,
    inputs: Vec<EntityId>,
}

struct TransactionResult {
    transaction_id: TransactionId,
    status: TransactionStatus,
}
```

## Version

`SX-CANON/1.0.6/STACK-v12` tooling package, level `L6.2`.
