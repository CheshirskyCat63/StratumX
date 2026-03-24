# compat_capabilities

## Role class
fact layer

## Canonical role
publishes or stores only capability fact data

## Owned data kind
capability fact only

## Operational meaning
capability facts only

## Non-authority
does not own versions, does not own profiles, does not own verdicts, does not own submission

## One data kind law
This layer owns exactly `capability fact` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: verdicts, gates, and boundary readers

Below: shared capability enums and runtime handles only where justified

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
