# l5.6-compat-profiles

## Canonical layer
- compat_profiles

## Role class
- fact layer

## Why this layer exists
freezes compatibility and transport profile truth separately from versions, capabilities, and gates

## What this layer owns
profile facts only

## What this layer does not do
does not own verdicts, does not own gates, does not own UI profiles, does not own execution state

## Adjacent above
verdicts, gates, and boundary readers

## Adjacent below
shared profile enums only

## Collapse guard
This layer exists because profile fact must remain separate from adjacent authorities.
