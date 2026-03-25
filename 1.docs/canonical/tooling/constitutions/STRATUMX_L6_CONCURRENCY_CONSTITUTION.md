# StratumX L6 Concurrency Constitution

Mutable editor truth is single-writer.
Snapshot, index, derived, artifact, validation, preview, and reporting work may parallelize only around immutable inputs or explicit job products.
Cross-thread mutation around the transaction ledger is forbidden.
