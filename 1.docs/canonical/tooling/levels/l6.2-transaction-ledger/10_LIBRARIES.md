# L6.2 Transaction Ledger Libraries

## Purpose

Owned library classes for transaction ledger level.

## Owned Classes

```rust
struct TransactionLedger {
    transactions: Vec<Transaction>,
    current_epoch: EpochId,
}

struct Transaction {
    id: TransactionId,
    command: CommandId,
    inputs: Vec<EntityId>,
    outputs: Vec<EntityId>,
    status: TransactionStatus,
}

enum TransactionStatus {
    Pending,
    Applied,
    Reverted,
}
```

## Version

`SX-CANON/1.0.6/STACK-v12` tooling package, level `L6.2`.
