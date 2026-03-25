# context_evidence_packs Level

Canonical layer: `context_evidence_packs`
Activation class: `warm`.

## Owns
- bounded evidence packs, redaction rules, freshness stamps, and evidence manifests

## Consumes
- L6 snapshots, diagnostics, refs, and explicit user/tool context

## Emits
- evidence pack ids and evidence pack payloads

## Never owns
- whole-project dumps or mutable editor mirrors
