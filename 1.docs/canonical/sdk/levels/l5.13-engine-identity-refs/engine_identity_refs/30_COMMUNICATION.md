# Communication

## Inputs
- public L4 identity export records only

## Outputs
- engine identity refs only

## Fan-in and fan-out
- fan-in allowed from the named lower truth sources only
- fan-out allowed to named boundary or reader consumers only
- no direct fan-out to engine unless this is a boundary layer

## Read/write mode
immutable fanout identity publication with single canonical ownership

## Local communication law
This layer expresses identity ref only. It never publishes editor workflow state or hidden execution runtime.
