# l5.7-compat-verdicts

## Canonical layer
- compat_verdicts

## Role class
- verdict layer

## Why this layer exists
publishes closed compatibility answers separately from facts and from per-action legality gates

## What this layer owns
compatibility verdicts only

## What this layer does not do
does not own versions, does not own capabilities, does not own gates, does not own submission

## Adjacent above
L6 capability readers and legality gates

## Adjacent below
compat version, capability, and profile facts

## Collapse guard
This layer exists because compatibility verdict must remain separate from adjacent authorities.
