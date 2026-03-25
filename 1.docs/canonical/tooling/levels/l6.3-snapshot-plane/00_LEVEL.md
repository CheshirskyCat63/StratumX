# snapshot_plane Level

Canonical layer: `snapshot_plane`
Activation class: `hot-read`.

## Owns
- immutable versioned editor read state

## Consumes
- authority deltas, L5 bridge snapshots, transaction invalidations

## Emits
- snapshot handles and versioned read views

## Never owns
- direct mutation authority
