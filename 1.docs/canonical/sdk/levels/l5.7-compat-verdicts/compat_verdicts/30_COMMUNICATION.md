# Communication

## Inputs
- version, capability, and profile facts

## Outputs
- compatibility verdicts only

## Fan-in and fan-out
- fan-in allowed from the named lower truth sources only
- fan-out allowed to named boundary or reader consumers only
- no direct fan-out to engine unless this is a boundary layer

## Read/write mode
parallel evaluation with singular immutable verdict publication

## Local communication law
This layer expresses compatibility verdict only. It never publishes editor workflow state or hidden execution runtime.
