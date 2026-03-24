# l5.5-compat-capabilities

## Canonical layer
- compat_capabilities

## Role class
- fact layer

## Why this layer exists
freezes capability truth separately from versions, profiles, and verdicts

## What this layer owns
capability facts only

## What this layer does not do
does not own versions, does not own profiles, does not own verdicts, does not own submission

## Adjacent above
verdicts, gates, and boundary readers

## Adjacent below
shared capability enums and runtime handles only where justified

## Collapse guard
This layer exists because capability fact must remain separate from adjacent authorities.
