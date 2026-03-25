# Communication

## Inputs
- public L4 artifact export records only
- observation anchors where justified

## Outputs
- engine artifact refs only

## Fan-in and fan-out
- fan-in allowed from the named lower truth sources only
- fan-out allowed to named boundary or reader consumers only
- no direct fan-out to engine unless this is a boundary layer

## Read/write mode
immutable fanout artifact-ref publication with single canonical ownership

## Local communication law
This layer expresses artifact ref only. It never publishes editor workflow state or hidden execution runtime.
