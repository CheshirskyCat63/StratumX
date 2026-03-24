# Result, Artifact, and Verdict Separation

## Canonical rule
L5 thin bridge owns verdicts only.
L5 thin bridge does not own artifacts.
L5 thin bridge does not own workflow results.

## Why this matters
Artifacts and workflow results belong to higher orchestration layers.
Compatibility verdicts and legality gates belong to L5 because they constrain boundary passage, not editor workflows.

## Consequence
Any proposal to add artifacts, preview outputs, generated deltas, task results, or release results to L5 is non-canonical and must be rejected.
