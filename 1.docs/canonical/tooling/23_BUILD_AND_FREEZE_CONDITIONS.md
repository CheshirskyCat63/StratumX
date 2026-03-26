
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
- the active evidence pack is present and authoritative
- `27_ACCEPTANCE_MATRIX.md` is fully `pass`
- `99_AUDIT_READINESS_MATRIX.md` contains zero open blockers
- `24_TESTING_MODEL.md` is closed by the active test-closure artifact
- active test-result artifact exists with executed test evidence (not just test-class coverage)

Any package revision missing the active evidence pack or the active test-closure artifact is not gold, even if the structural docs look complete.
