# l5.8-transport-policies

## Canonical layer
- transport_policies

## Role class
- policy layer

## Why this layer exists
keeps transport policy truth separate from profiles, gates, and boundary envelopes

## What this layer owns
transport policy refs only

## What this layer does not do
does not own profiles, does not own gates, does not own queues, does not own retries

## Adjacent above
profiles, gates, and boundary readers

## Adjacent below
shared policy enums only

## Collapse guard
This layer exists because policy ref must remain separate from adjacent authorities.
