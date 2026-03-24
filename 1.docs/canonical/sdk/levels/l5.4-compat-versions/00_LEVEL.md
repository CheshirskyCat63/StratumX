# l5.4-compat-versions

## Canonical layer
- compat_versions

## Role class
- fact layer

## Why this layer exists
freezes version truth separately from capabilities, profiles, and verdicts

## What this layer owns
version compatibility facts only

## What this layer does not do
does not own capabilities, does not own profiles, does not own verdicts, does not own submission

## Adjacent above
verdicts, gates, and boundary readers

## Adjacent below
shared registry ids and package stack version only

## Collapse guard
This layer exists because version fact must remain separate from adjacent authorities.
