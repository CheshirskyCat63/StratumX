# Communication

## Inputs
- declared compatibility and transport profile facts

## Outputs
- profile facts only

## Fan-in and fan-out
- fan-in allowed from declared package truth only
- fan-out allowed to verdicts, gates, boundary readers, and runtime readers as justified
- no direct fan-out to engine

## Read/write mode
low-churn single-writer fact publication

## Local communication law
This layer expresses profile fact only. It never publishes editor workflow state or hidden execution runtime.
