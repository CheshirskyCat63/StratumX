# Field Contract Rules

## Requiredness law
- `required` means the field must exist in every normalized record of the layer.
- `optional` means the field may be absent but, when present, must obey the layer invariant.

## Invariant law
- ids must remain stable within the scope named by their layer contract.
- enums must use the closed shared registry enums.
- refs and handles must remain opaque upward.
- optional anchor or reason refs must never smuggle payload bodies or UI text into L5.

## Cross-layer field law
- boundary layers may carry order keys, class enums, payload-type enums, and opaque handles only.
- fact layers may carry ids, enums, flags, and refs to policies only where justified.
- verdict and gate layers may carry reason refs only; they may not carry recovery plans.
- handle/ref layers may point to exported identities or observation anchors only when the local layer contract names them explicitly.
