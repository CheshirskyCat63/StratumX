# Glossary

authority core: tiny single-writer editor truth.

command envelope: canonical mutation request package.

transaction ledger: ordered commit, rollback, apply, and revert truth for editor mutations.

snapshot: immutable versioned read plane.

index: rebuildable lookup plane.

derived projection: disposable non-authority view.

artifact: deterministic generated product with manifest identity and rebuild rules.

stream: bounded forward-only event, metrics, or diagnostics flow.

cache: evictable non-authoritative acceleration storage.

budget runtime: layer that owns cost envelopes, pressure visibility, deny/defer/degrade decisions, and degradation policy enforcement.

context evidence pack: bounded project/editor evidence sent toward assistant runtime or reasoning.

proposal: assistant-suggested change bundle before legal lowering.

lowering: translation from proposal or plan into typed `L6` commands.

campaign bundle: compiled studio orchestration package.

plan bundle: planning output from `L7A` before lowering.

hot spine: always-active ownership path `L5 -> L6`.

warm bridge: request-driven assistant runtime path `L6A`.

cold control spine: planning and orchestration path `L7` and `L7A`.
