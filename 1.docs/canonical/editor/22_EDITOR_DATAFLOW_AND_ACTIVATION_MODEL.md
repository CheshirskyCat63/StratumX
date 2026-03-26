# Editor Dataflow and Activation Model

## Product data classes
- shell/layout data
- viewport navigation and camera data
- selection presentation state
- tool context and overlay state
- panel/view state
- browser/search/filter/index state
- inspector presentation and staged edit state
- suite-local UI state
- diagnostics presentation data
- build/release presentation and task projection data
- collaboration/review presentation data

## Flow classes
### truth flow
All authoritative mutation requests descend to `L6`.

### assistant flow
Assistant requests descend through `L6A`; planning-only surfaces descend through `L7A`.

### orchestration flow
Batch and orchestration requests descend through `L7`.

### preview flow
Preview and speculative requests descend as preview work and return discardable projections.

### diagnostics flow
Warnings, traces, metrics, validation findings, and budget signals ascend as read projections.

### collaboration flow
Comments, review markers, approvals, and capture metadata flow through `L11` surfaces and legal lower-stack hooks.

## Activation law
Only the active viewport, focused interaction context, visible panels, active suite, and running jobs consume hot resources.
Inactive suites and closed views are cold.
Background indexing, import, compile, capture, and validation jobs are batch-bounded and pressure-visible.
