# Build And Freeze Conditions

The tools stack is frozen only when:
- `L6 authority_core` remains tiny and single-writer
- all editor mutation paths go through `L6` commands and transactions
- command classes, invalidation classes, and transaction outputs are explicit
- every heavy domain answers authority/snapshot/index/derived/artifact/cache separation
- artifact plane is explicit and distinct from cache and derived
- build runtime is explicit and distinct from release runtime
- budget runtime is explicit and enforces pressure policy
- `L6A` has no direct mutation authority and no shadow world
- `L7A` has no direct apply authority
- `L7` remains cold compiled-control only
- all shortcut paths are denied
- memory/GPU/disk discipline is explicit
- acceptance rows and evidence rows for `L6`, `L6A`, `L7`, and `L7A` are closed
