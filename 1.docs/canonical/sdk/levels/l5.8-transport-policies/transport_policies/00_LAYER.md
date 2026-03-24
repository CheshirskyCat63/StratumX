# transport_policies

## Role class
policy layer

## Canonical role
publishes or stores only policy ref data

## Owned data kind
policy ref only

## Operational meaning
transport policy refs only

## Non-authority
does not own profiles, does not own gates, does not own queues, does not own retries

## One data kind law
This layer owns exactly `policy ref` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: profiles, gates, and boundary readers

Below: shared policy enums only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
