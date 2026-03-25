# assistant_sessions Level

Canonical layer: `assistant_sessions`
Activation class: `warm`.

## Owns
- assistant session lifecycle, attached context handles, and active run state

## Consumes
- assistant requests, workspace session refs

## Emits
- session ids and session state summaries

## Never owns
- editor truth or planning truth
