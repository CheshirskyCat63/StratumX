# Communication

## Inputs
- compat verdicts
- capability facts
- profiles
- transport policy refs
- runtime/object handles when justified

## Outputs
- legality gate verdicts only

## Fan-in and fan-out
- fan-in allowed from the named lower truth sources only
- fan-out allowed to named boundary or reader consumers only
- no direct fan-out to engine unless this is a boundary layer

## Read/write mode
parallel evaluation with singular immutable gate publication per target action

## Local communication law
This layer expresses legality gate only. It never publishes editor workflow state or hidden execution runtime.
