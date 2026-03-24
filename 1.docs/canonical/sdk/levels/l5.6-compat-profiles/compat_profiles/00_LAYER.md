# compat_profiles

## Role class
fact layer

## Canonical role
publishes or stores only profile fact data

## Owned data kind
profile fact only

## Operational meaning
profile facts only

## Non-authority
does not own verdicts, does not own gates, does not own UI profiles, does not own execution state

## One data kind law
This layer owns exactly `profile fact` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: verdicts, gates, and boundary readers

Below: shared profile enums only

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
