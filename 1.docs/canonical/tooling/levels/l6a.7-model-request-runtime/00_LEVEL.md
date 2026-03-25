# model_request_runtime Level

Canonical layer: `model_request_runtime`
Activation class: `warm-io`.

## Owns
- bounded model requests, responses, cancellations, timeout classes, and retry classes

## Consumes
- evidence packs, goal requests, proposal generation requests, routing decisions

## Emits
- model responses and model runtime traces

## Never owns
- plan truth, editor truth, or unbounded async IO
