# Testing Model

## Surface tests
- shell layout persistence
- panel activation/deactivation
- workspace save/restore
- command palette discovery
- shortcut conflict detection

## View and interaction tests
- viewport navigation
- selection sync
- inspector edit routing
- drag/drop legality
- outliner/world-browser sync

## Suite tests
- suite activation cost
- overlay correctness
- preview disposal
- diagnostics visibility
- artifact handoff visibility

## Resource tests
- hidden panel no-tick
- inactive suite no-rebuild
- bounded thumbnail cache
- bounded log/diagnostics history

## Integration tests
- user action -> lower-stack command path
- assistant proposal -> apply/revert visibility
- build task -> artifact visibility
- collaboration note -> review trace visibility
