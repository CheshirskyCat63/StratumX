# L6.2 Transaction Ledger Fields

## Owned State

```rust
struct LedgerState {
    transactions: Vec<Transaction>,
    current_epoch: EpochId,
    pending_count: usize,
}
```

## Version

`SX-CANON/1.0.6/STACK-v12` tooling package, level `L6.2`.
