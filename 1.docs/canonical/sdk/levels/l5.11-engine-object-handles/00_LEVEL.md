# l5.11-engine-object-handles

## Canonical layer
- engine_object_handles

## Role class
- handle/ref layer

## Why this layer exists
isolates engine object handles from session handles, runtime handles, identity refs, and state refs

## What this layer owns
object handles only

## What this layer does not do
does not own sessions, does not own state, does not own object metadata, does not own editor objects

## Adjacent above
boundary, gates, and L6 readers only

## Adjacent below
public L4 object handle surfaces only

## Collapse guard
This layer exists because object handle ref must remain separate from adjacent authorities.
