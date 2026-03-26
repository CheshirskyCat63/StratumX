# Global Authority Order

## Authority Hierarchy

The following is the strict, unambiguous authority order for resolving conflicts within the StratumX canonical stack. Higher items override lower items.

| Authority Level | Source | Scope | Overrides |
|-----------------|--------|-------|-----------|
| 1 | Constitutional Law (Engine) | Engine-specific domains (performance, memory, benchmark, etc.) | All global and package documents when domain-specific |
| 2 | `05_GLOBAL_AUTHORITY_ORDER.md` | Global stack-wide | All package root documents and package-specific laws |
| 3 | Package Constitution/Root Laws | Package-specific (engine constitutions and each package root law set; sdk/tooling/editor do not define a standalone constitution file) | Package acceptance matrices and local documents within that package |
| 4 | `06_GLOBAL_ACCEPTANCE_MATRIX.md` | Global acceptance criteria | Package acceptance matrices when defining gold readiness |
| 5 | Package Acceptance Matrix | Package-specific acceptance criteria | Package evidence registry and readiness matrix within that package |
| 6 | `07_GLOBAL_EVIDENCE_REGISTRY.md` | Global active evidence artifacts | Local evidence references when defining active artifacts |
| 7 | Package Evidence Registry | Package-specific evidence artifacts | Package readiness matrix when validating evidence |
| 8 | `99_GLOBAL_AUDIT_READINESS_MATRIX.md` | Global readiness verification | Local readiness assessments when determining stack-wide gold |
| 9 | Package Readiness Matrix | Package-specific readiness assessment | Local notes and observations within that package |
| 10 | Local Notes/Observations | Package-specific annotations | None (lowest authority) |

## Authority Resolution Principles

1. **Engine Constitutional Supremacy**: When an engine constitution addresses a specific domain (e.g., performance, memory safety), it overrides global documents for that domain only.

2. **Global Authority Supremacy**: `05_GLOBAL_AUTHORITY_ORDER.md` overrides all package-level documents when defining stack-wide relationships, dependencies, and boundaries.

3. **Package Constitutional Authority**: Within a package, its constitution overrides its own acceptance and evidence documents for package-specific matters.

4. **Acceptance Matrix Supremacy**: Global acceptance matrix (`06_GLOBAL_ACCEPTANCE_MATRIX.md`) defines what constitutes gold readiness for the entire stack, overriding package acceptance matrices when they conflict on gold criteria.

5. **Evidence Registry Supremacy**: Global evidence registry (`07_GLOBAL_EVIDENCE_REGISTRY.md`) defines what constitutes an active artifact for the entire stack, overriding package evidence registries when they conflict on artifact status.

6. **Readiness Matrix Supremacy**: Global audit readiness matrix (`99_GLOBAL_AUDIT_READINESS_MATRIX.md`) defines verification of readiness for the entire stack, overriding package readiness matrices when they conflict on readiness assessment.

## Conflict Resolution Process

When a conflict arises between documents:
1. Identify the authority level of each conflicting document using the table above
2. The document with the higher authority level prevails
3. If both documents are at the same authority level, the more specific document (package-level over global for package matters) prevails
4. If still unresolved, escalate to engine constitutional law if domain-specific, otherwise maintain status quo and flag for constitutional review

## Package-Specific Authority Documents

Each package must maintain an explicit authority-root set composed of:
- package index
- scope
- stack or layer map
- role map
- dependency model
- boundary preservation matrix
- acceptance matrix
- evidence registry
- build and freeze conditions
- document authority order
- audit readiness matrix

Concrete ordinals differ by package and must be taken from the local package root, not assumed from a fake shared numbering scheme.

These documents derive their authority from their position in the hierarchy above and must not contradict higher-authority documents.