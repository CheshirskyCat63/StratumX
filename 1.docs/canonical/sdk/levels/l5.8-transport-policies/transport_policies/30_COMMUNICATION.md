# Communication

## Inputs
- declared bridge transport policy records only

## Outputs
- transport policy refs only

## Fan-in and fan-out
- fan-in allowed from declared package truth only
- fan-out allowed to verdicts, gates, boundary readers, and runtime readers as justified
- no direct fan-out to engine

## Read/write mode
low-churn single-writer immutable policy publication

## Local communication law
This layer expresses policy ref only. It never publishes editor workflow state or hidden execution runtime.
