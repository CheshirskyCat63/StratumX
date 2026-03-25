# StratumX L6 Authority And Transaction Constitution

`authority_core` is tiny, single-writer, and minimal.
Every mutation enters through typed commands and commits through the transaction ledger.
Undo, redo, assistant apply, and assistant revert are legal only when transaction-bound, auditable, and invalidation-visible.
