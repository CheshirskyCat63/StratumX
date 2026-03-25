# Family Level

Canonical family: `pipeline_and_graph_family`

## Composes
- bootstrap, import/export, graph systems, templates, and dependency services

## Data responsibility
- authority-facing minimal truth: service authority refs only
- snapshot classes: service snapshots
- index classes: service lookup indices
- derived classes: derived pipeline and graph views
- artifact classes: import/export/graph artifacts
- preview classes: pipeline previews where disposable
- cache classes: service caches only
- diagnostics classes: pipeline and graph diagnostics
- degradation priority: `medium`

This family composes editor product surfaces without owning hidden lower-stack truth.
