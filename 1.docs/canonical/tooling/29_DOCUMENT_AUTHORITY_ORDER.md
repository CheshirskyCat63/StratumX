
# Document Authority Order

## Authority order
1. constitutions
2. root documents
3. level contracts
4. family contracts
5. active evidence pack
6. archival notes and superseded evidence records

## Active evidence pack for this package version
The active evidence pack is:
- `evidence/root/root_traceability_log_v3.md`
- `evidence/layers/layer_traceability_log_v3.md`
- `evidence/root/tooling_acceptance_evidence_v3.md`
- `evidence/tests/tooling_test_closure_v3.md`
- `evidence/tests/tooling_test_result_posture_v3.md`

## Pre-v3 rule
Package revisions before `v3` did not contain an authoritative evidence pack for current closure.
They are superseded by the active pack above and may not be used to claim current acceptance or readiness.

## Derivative rule
`27_ACCEPTANCE_MATRIX.md` and `99_AUDIT_READINESS_MATRIX.md` may claim `pass` only through the active evidence pack above.
They must not backfill missing proof from superseded package revisions.

## Conflict rule
If an archival note disagrees with the active evidence pack, the archival note loses authority immediately.
If the active evidence pack disagrees with constitutions, root docs, or local contracts, the evidence pack is wrong and must be regenerated.
