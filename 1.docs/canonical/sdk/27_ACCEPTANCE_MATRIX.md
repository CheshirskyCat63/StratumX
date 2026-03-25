# Acceptance Matrix

| Row | Requirement | Status |
|---|---|---|
| ROOT-001 | all L5 levels present, including complete local docs for `l5.15-engine-artifact-refs` | pass |
| ROOT-002 | controls split from packets | pass |
| ROOT-003 | facts split from verdicts | pass |
| ROOT-004 | artifact refs split from state refs | pass |
| ROOT-005 | public L4 synchronization is explicit as sinks, snapshots, batches, and invalidation signals | pass |
| ROOT-006 | physical data layout model is explicit | pass |
| ROOT-007 | batch and cursor publication model is explicit | pass |
| ROOT-008 | compiled gate and compatibility lookup model is explicit | pass |
| ROOT-009 | L5 to L6 snapshot and stream alignment is explicit | pass |
| ROOT-010 | handle/ref opacity law explicit | pass |
| ROOT-011 | no hidden store law explicit | pass |
| ROOT-012 | testing model covers lookup correctness, pressure, replay, snapshot swap, and allocation posture | pass |
| ROOT-013 | build handoff keeps crate split narrow and physically non-overlapping | pass |
| ROOT-014 | tools consumption map is explicit by plane and access type | pass |
| ROOT-015 | hot-path allocation and locality law is explicit | pass |
