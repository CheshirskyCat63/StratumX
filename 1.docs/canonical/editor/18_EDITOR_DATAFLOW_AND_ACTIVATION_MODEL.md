# Editor Dataflow and Activation Model

## Product data classes
- shell/layout data
- viewport state
- selection presentation state
- tool context state
- panel/view state
- browser/search/filter state
- inspector presentation state
- suite-local UI state
- diagnostics presentation data
- collaboration/review presentation data

## Routing
- all mutation requests descend to `L6`
- assistant requests descend through `L6A`
- orchestration requests descend through `L7`
- planning surfaces descend through `L7A`

## Activation law
Only the active viewport, active suite, visible panels, and running jobs consume hot resources.
Inactive suites and closed views are cold.
