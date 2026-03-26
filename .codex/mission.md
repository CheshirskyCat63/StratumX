# Mission

Build the StratumX engine implementation from the canonical stack without violating package boundaries or inventing architecture.

## Primary objective
Turn the canonical documentation into a real Rust workspace with:
- compilable crates
- enforced package boundaries
- tests
- smoke execution
- benchmarks
- machine-readable metrics
- auditable reports

## Current operating mode
- Canon first
- Code second
- Metrics required
- No fiction
- No hidden scope expansion
- No canon edits unless explicitly assigned

## Execution contract
For every task:
1. read canon
2. identify exact target package and target layer
3. implement only the assigned slice
4. add or update tests
5. run repository scripts
6. write the result using `.codex/report-template.md`

## Stop conditions
Stop immediately and report a blocker if:
- canon is missing required authority for the assigned change
- task requires architecture invention outside canon
- package boundaries would be violated
- required checks cannot run
- environment is broken