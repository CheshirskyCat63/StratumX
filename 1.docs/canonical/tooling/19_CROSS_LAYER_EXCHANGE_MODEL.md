# Cross-Layer Exchange Model

## Canonical downward paths
- `user/tool -> L6 -> L5 -> engine`
- `user/tool -> L6A -> L6 -> L5 -> engine`
- `user/tool -> L7A -> L6A -> L6 -> L5 -> engine`
- `L7 -> L6 -> L5 -> engine`

## Canonical upward paths
- `engine -> L5 -> L6`
- `L6 -> L6A` for proposal/apply/runtime views
- `L6A -> L7A` for bounded planning and reasoning requests
- `L6 -> L7` for campaign/reporting feedback only when explicitly requested

## Canonical downward classes
- command envelope
- binding control
- execution signal
- preview session request
- build request
- release request
- artifact bind request
- campaign task bundle
- governance policy bundle
- automation request bundle
- plan bundle
- proposal intent bundle

## Canonical upward classes
- transaction result
- invalidation set
- immutable snapshot handle
- index handle
- observation batch
- metric batch
- artifact manifest
- pressure frame
- compatibility verdict
- legality verdict
- safety verdict
- opaque handle
- opaque ref
- evidence record

No hidden shortcut path is allowed.
No upper layer may widen or reshape `L5` bridge truth into editor truth without an explicit `L6` plane contract.
