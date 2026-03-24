# Communication

## Inputs
- typed control request publications
- compat and gate reads
- runtime/object/session handle reads

## Outputs
- control envelopes only

## Fan-in and fan-out
- fan-in allowed from L6 request publishers only
- fan-out allowed only to public L4 surfaces
- no direct fan-out to L6

## Read/write mode
ordered single-writer submission per engine session and per control domain

## Local communication law
This layer expresses control boundary only. It never publishes editor workflow state or hidden execution runtime.
