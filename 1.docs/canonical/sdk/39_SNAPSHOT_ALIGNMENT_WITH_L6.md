# Snapshot Alignment With L6

## Canonical alignment
- L5 immutable bridge snapshots feed L6 snapshot-plane consumers
- L5 immutable egress batches feed L6 stream-plane consumers
- L5 ordered ingress lanes accept L6 command-plane publication
- L5 artifact refs feed L6 artifact-plane readers without transferring build ownership

## Law
L6 may derive editor truth from L5 publications, but L5 may not pre-own editor truth on behalf of L6.
L6 may cache derived views, but L5 remains limited to the boundary substrate.

## Forbidden
- L6 bypassing L5 to reach engine internals
- L5 reshaping itself into editor authority state
- duplicate mutable truth stores spanning both layers
