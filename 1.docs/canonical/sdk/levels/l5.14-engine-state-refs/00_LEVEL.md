# l5.14-engine-state-refs

## Canonical layer
- engine_state_refs

## Role class
- handle/ref layer

## Why this layer exists
isolates exported state refs from handles, identities, observations, and tooling projections

## What this layer owns
engine state refs only

## What this layer does not do
does not own state payload bodies, does not own observations, does not own projections, does not own mutation

## Adjacent above
L6 readers only

## Adjacent below
public L4 state export surfaces and observation anchors only

## Collapse guard
This layer exists because state ref must remain separate from adjacent authorities.
