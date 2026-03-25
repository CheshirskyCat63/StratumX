# L5 To Tools Consumption Map

| Upper layer | L5 modules consumed | Access type | Notes |
|---|---|---|---|
| L6 editor operating layer | ingress lanes, egress batches, compat snapshots, lookup verdicts, legality tables, handles, refs, artifact refs | direct typed bridge consumption by plane | editor truth remains in L6 |
| L6A assistant runtime | observations, metrics, bounded handles/refs via L6-shaped evidence | indirect bounded stream and evidence consumption | no direct L6A ownership leak into L5 |
| L7 studio orchestration | profile facts, compatibility verdicts, bounded metrics, bounded artifact refs via L6 surfaces | bounded meta consumption | orchestration stays above L6 |
| L7A assistant brain | none directly by default | indirect only | planning must not couple to raw bridge internals |
