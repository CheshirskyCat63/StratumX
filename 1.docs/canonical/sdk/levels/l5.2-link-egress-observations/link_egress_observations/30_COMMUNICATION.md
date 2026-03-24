# Communication

## Inputs
- public L4 observation envelopes only

## Outputs
- observation envelopes only

## Fan-in and fan-out
- fan-in allowed only from public L4 surfaces
- fan-out allowed to L6 readers only
- no direct fan-out to engine

## Read/write mode
immutable fanout publication with single canonical boundary ownership

## Local communication law
This layer expresses observation boundary only. It never publishes editor workflow state or hidden execution runtime.
