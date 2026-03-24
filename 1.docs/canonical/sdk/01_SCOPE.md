# Scope

## In scope
This package defines the full canonical thin L5 runtime truth bridge for StratumX:
- the write-side boundary mesh
- the read-side boundary mesh
- compatibility facts and verdicts
- transport policies and legality gates
- runtime-facing handles and opaque refs
- root semantic locks
- package layout
- constitutions
- per-level canonical guards
- per-layer local contracts
- synchronization with public engine L4 surfaces
- build and freeze conditions
- testing model for code-generation readiness

## Out of scope
This package does not define:
- L4 engine internals
- L6 tooling behavior, shells, previews, imports, commands, or AI workflows
- L7 studio/product workflows
- project, module, asset, template, or recipe semantics above the runtime bridge
- UI rendering, skinning, themes, panels, or viewport composition

## Primary goal
A reader must be able to answer:
- which L5 layers are allowed to touch public L4 surfaces
- which facts and verdicts constrain write-side publication
- which runtime-facing handles are exported upward
- what data kinds are intentionally forbidden in L5
- how L5 is tested for mechanical and semantic correctness
