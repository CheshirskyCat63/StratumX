# Global Build and Freeze Conditions

## Purpose
This document defines the conditions under which implementation-start, package-complete, and gold-freeze operations are permitted within the StratumX canonical stack.

## Implementation-Start Conditions
Implementation-start is allowed when:
- The package's root documents (00_INDEX.md through 03_PACKAGE_ROLE_MAP.md) exist and are internally consistent
- The package's dependency model does not violate the global dependency model
- No constitutional law prohibits implementation in the affected domain

## Package-Complete Conditions
Package-complete is allowed when:
- All acceptance matrix rows for the package are in `pass` status
- Every `pass` row references an active artifact in the package's evidence registry
- The package's evidence registry contains only active artifacts that exist
- No duplicate ordinals exist in the package's root documents
- No wildcard/glob authority references exist where exact canonical paths are required
- All hot/core layers have full local documentation packages (not single-file contracts)

## Gold-Freeze Conditions
Global gold-freeze is allowed ONLY when:
- Every package in the stack has achieved package-complete status
- Every row in the Global Acceptance Matrix (`06_GLOBAL_ACCEPTANCE_MATRIX.md`) is in `pass` status
- Every `pass` row in the Global Acceptance Matrix references an active artifact in the Global Evidence Registry (`07_GLOBAL_EVIDENCE_REGISTRY.md`)
- The Global Evidence Registry contains only active artifacts that exist
- The Global Audit Readiness Matrix (`99_GLOBAL_AUDIT_READINESS_MATRIX.md`) shows full alignment with global acceptance and evidence
- The stack-wide version marker (`STACK_VERSION`) is present and correctly formatted as `SX-CANON/1.0.6/STACK-v12`
- No package has outstanding constitutional violations that affect stack-wide integrity

## Prohibited Conditions
Global gold-freeze is PROHIBITED if:
- Any package has not achieved package-complete status
- Any row in the Global Acceptance Matrix is not in `pass` status
- Any `pass` row in the Global Acceptance Matrix references a non-existent or inactive artifact
- The Global Evidence Registry references any non-existent artifact
- Any package contains duplicate ordinals in its root documents
- Any package uses wildcard/glob authority references where exact canonical paths are required
- Any hot/core layer relies on a single-file contract instead of full local documentation
- Any package has violated the global dependency model
- Any package has violated the global authority order
- Any package exhibits forbidden direct ownership or shadow truth
- The stack-wide version marker is missing or not formatted as `SX-CANON/1.0.6/STACK-v12`

## Verification Process
Before declaring global gold-freeze:
1. Verify each package's internal completion using its own acceptance/evidence/readiness matrices
2. Verify global acceptance matrix compliance
3. Verify global evidence registry integrity
4. Verify global audit readiness matrix alignment
5. Confirm stack-wide version marker validity
6. Confirm no constitutional violations affect stack-wide integrity

## Emergency Unfreeze Conditions
If after gold-freeze any of the following are discovered:
- Evidence artifact referenced in acceptance matrix is deleted or becomes inactive
- Constitutional law is enacted that contradicts accepted package truths
- Dependency model violation is discovered
- Authority order violation is discovered
Then the stack must immediately revert to non-frozen state and enter remediation process.