# engine_artifact_refs

## Role class
handle/ref layer

## Canonical role
publishes or stores only artifact ref data

## Owned data kind
artifact ref only

## Operational meaning
engine artifact refs only

## Non-authority
does not own build ownership, does not own artifact payload bodies, does not own state refs, does not own mutation

## One data kind law
This layer owns exactly `artifact ref` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: L6 readers only

Below: public L4 artifact export surfaces and observation anchors only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
