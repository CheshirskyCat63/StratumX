# l5.10-engine-session-handles

## Canonical layer
- engine_session_handles

## Role class
- handle/ref layer

## Why this layer exists
isolates engine session handles from object handles, runtime handles, identity refs, and state refs

## What this layer owns
session handles only

## What this layer does not do
does not own runtime surfaces, does not own object handles, does not own state, does not own UI sessions

## Adjacent above
boundary, facts, and L6 readers only

## Adjacent below
public L4 session handle surfaces only

## Collapse guard
This layer exists because session handle ref must remain separate from adjacent authorities.
