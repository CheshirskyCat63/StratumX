# Result, Artifact, Verdict Separation

A result is the outcome of an operation.
An artifact is a deterministic generated product.
A verdict is a decision about admissibility or legality.

These three classes must never be merged into one payload type.
Merging them causes cache poisoning, replay ambiguity, and bad ownership inference.
