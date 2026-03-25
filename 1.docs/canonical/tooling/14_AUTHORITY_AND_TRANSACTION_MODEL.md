# Authority And Transaction Model

`L6 authority_core` is tiny, single-writer, and never bulk-heavy.
Every editor mutation must enter through `command_envelopes` and commit through `transaction_ledger`.
Undo, redo, assistant apply, and assistant revert remain inside the transaction law only.

## Command envelope minimum fields
- `command_id`
- `command_class`
- `target_scope`
- `target_ref_set`
- `authority_touch_class`
- `previewability_class`
- `approval_class`
- `budget_class`
- `origin_class`

## Transaction ledger minimum outputs
- `transaction_id`
- `admission_verdict`
- `commit_order`
- `rollback_binding`
- `snapshot_invalidation_set`
- `index_invalidation_set`
- `derived_invalidation_set`
- `artifact_invalidation_set`
- `stream_publication_set`
- `evidence_record`

Bulk domain data must live as immutable snapshots, rebuildable indices, deterministic artifacts, or bounded streams, not inside `authority_core`.
