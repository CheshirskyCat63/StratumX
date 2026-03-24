# Assistant Semantic Separation

## Canonical rule
L5 owns no assistant semantics.

## What is forbidden in L5
- assistant intents
- assistant plans
- assistant context assembly
- assistant proposals
- assistant applies
- assistant verification
- assistant rollback
- provider routing
- model budgets
- prompt history

## Allowed relation
L5 may gate or reject a write-side boundary action requested by an assistant indirectly through L6.
That does not make assistant state an L5-owned domain.
