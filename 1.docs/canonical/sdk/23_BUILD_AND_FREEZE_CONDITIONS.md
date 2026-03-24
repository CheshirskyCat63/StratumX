# Build and Freeze Conditions

## Document freeze
This package may be treated as frozen only when:
- `27_ACCEPTANCE_MATRIX.md` shows every blocking row as `pass`
- `99_AUDIT_READINESS_MATRIX.md` shows no open blockers
- `11_PACKAGE_LAYOUT.md` exactly matches the real package
- `26_SHARED_TYPE_REGISTRY.md` covers every shared type used by every layer

## Implementation freeze
Code may be treated as conformant only when:
- every layer has an implementation surface matching its owned data kind
- no forbidden dependency shortcut exists
- no forbidden cross-layer mutation exists
- boundary ordering and fanout rules are mechanically tested
