# compat_verdicts

## Role class
verdict layer

## Canonical role
publishes or stores only compatibility verdict data

## Owned data kind
compatibility verdict only

## Operational meaning
compatibility verdicts only

## Non-authority
does not own versions, does not own capabilities, does not own gates, does not own submission

## One data kind law
This layer owns exactly `compatibility verdict` and must not absorb adjacent semantic classes.

## Adjacent distinction
Above: L6 capability readers and legality gates

Below: compat version, capability, and profile facts

## Forbidden drift
must not absorb adjacent semantic classes; must not become editor orchestration; must not become engine execution logic
