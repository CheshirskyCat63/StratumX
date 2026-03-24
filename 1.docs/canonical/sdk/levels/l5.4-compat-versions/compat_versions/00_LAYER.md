# compat_versions

## Role class
fact layer

## Canonical role
publishes or stores only version fact data

## Owned data kind
version fact only

## Operational meaning
version compatibility facts only

## Non-authority
does not own capabilities, does not own profiles, does not own verdicts, does not own submission

## One data kind law
This layer owns exactly `version fact` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: verdicts, gates, and boundary readers

Below: shared registry ids and package stack version only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
