# Communication

## Inputs
- typed packet request publications
- compat and gate reads
- runtime/session handle reads

## Outputs
- packet envelopes only

## Fan-in and fan-out
- fan-in allowed from L6 request publishers only
- fan-out allowed only to public L4 surfaces
- no direct fan-out to L6

## Read/write mode
ordered single-writer submission per engine session and per packet stream

## Local communication law
This layer expresses packet boundary only. It never publishes editor workflow state or hidden execution runtime.
