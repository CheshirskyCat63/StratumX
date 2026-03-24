# Communication

## Inputs
- public L4 runtime surface handles only

## Outputs
- runtime surface handles only

## Fan-in and fan-out
- fan-in allowed from the named lower truth sources only
- fan-out allowed to named boundary or reader consumers only
- no direct fan-out to engine unless this is a boundary layer

## Read/write mode
immutable fanout handle publication with single canonical ownership

## Local communication law
This layer expresses runtime handle ref only. It never publishes editor workflow state or hidden execution runtime.
