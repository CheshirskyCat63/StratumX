# L4 Synchronization Model

## Public-surface-only rule
L5 synchronizes only with public L4 packet, control, observation, metric, session-handle, object-handle, runtime-handle, identity-export, and state-export surfaces.

## Synchronization law
- write-side synchronization preserves per-session order.
- read-side synchronization preserves source emission order.
- exported handles and refs are published immutably upward.
- L5 never reaches below public L4 contracts to reconstruct hidden runtime state.

## Drift guard
Any proposed dependency on non-public L4 internals is a package-level blocker.
