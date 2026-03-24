# l5.12-engine-runtime-handles

## Canonical layer
- engine_runtime_handles

## Role class
- handle/ref layer

## Why this layer exists
isolates public runtime-surface handles from sessions, objects, identity refs, and state refs

## What this layer owns
runtime surface handles only

## What this layer does not do
does not own sessions, does not own observations, does not own metrics, does not own runtime scheduling

## Adjacent above
boundary, facts, gates, and L6 readers only

## Adjacent below
public L4 runtime surface handles only

## Collapse guard
This layer exists because runtime handle ref must remain separate from adjacent authorities.
