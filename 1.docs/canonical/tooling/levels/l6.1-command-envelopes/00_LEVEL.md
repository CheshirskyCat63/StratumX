# command_envelopes Level

Canonical layer: `command_envelopes`
Activation class: `hot`.

## Owns
- typed mutation requests, target scopes, approval classes, origin classes, budget classes

## Consumes
- tool requests, user actions, lowered assistant commands, compiled campaign tasks

## Emits
- transaction candidates

## Never owns
- editor truth, snapshots, hidden side effects
