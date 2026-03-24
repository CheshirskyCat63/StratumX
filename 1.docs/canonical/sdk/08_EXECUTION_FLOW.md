# Execution Flow

## Write-side flow
1. L6 publishes a typed request.
2. L5 reads compatibility facts, transport policy, legality gate, and required runtime handles.
3. L5 emits either a packet envelope or a control envelope through the write-side boundary mesh.
4. Public L4 consumes the submitted envelope.

## Read-side flow
1. Public L4 emits observations and metrics.
2. L5 republishes them immutably through the read-side boundary mesh.
3. L5 also republishes compatible runtime-facing handles and exported refs upward.
4. L6 consumes observations, metrics, handles, and refs to build its own projections and workflows.

## Hard rule
L5 never plans editor work, never performs preview generation, never owns content import, and never owns assistant orchestration.
