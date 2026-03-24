# l5.9-legality-gates

## Canonical layer
- legality_gates

## Role class
- gate layer

## Why this layer exists
answers per-action legality questions separately from compatibility verdicts and boundary envelopes

## What this layer owns
boundary legality gate verdicts only

## What this layer does not do
does not own compatibility truth, does not own retries, does not own boundary publication, does not own UI policy

## Adjacent above
boundary publishers only

## Adjacent below
compat verdicts, capability facts, profiles, and transport policies

## Collapse guard
This layer exists because legality gate must remain separate from adjacent authorities.
