# l5.13-engine-identity-refs

## Canonical layer
- engine_identity_refs

## Role class
- handle/ref layer

## Why this layer exists
isolates stable engine-facing identity refs from object handles and state refs

## What this layer owns
engine identity refs only

## What this layer does not do
does not own handles, does not own state, does not own editor-facing asset identities, does not own mutation

## Adjacent above
L6 readers, object-handle readers, and state-ref readers only

## Adjacent below
public L4 identity export surfaces only

## Collapse guard
This layer exists because identity ref must remain separate from adjacent authorities.
