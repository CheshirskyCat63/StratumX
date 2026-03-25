# Library Baseline

Allowed classes by layer:
- `L6`: transaction-safe, snapshot-safe, index-safe, artifact-safe, bounded-stream, bounded-cache, and explicit preview/build/release libraries
- `L6A`: bounded async model IO, proposal/lowering/safety tooling, and evidence-pack assembly libraries
- `L7`: campaign scheduling, workflow graph, governance, and reporting libraries
- `L7A`: planning, routing, reasoning, and structured-output libraries

Library law:
- no library may silently introduce hidden authority stores
- no library may introduce unbounded queues or unbounded caches on hot or warm paths
- no library may create uncontrolled GPU residency or silent disk growth
- no library may smuggle editor mutation authority around `L6`
- no library may force intra-process serialization when typed in-memory exchange already exists
