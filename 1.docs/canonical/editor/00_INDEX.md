# StratumX Editor Product Canon Index

This package defines the editor as a product layer above the canonical tools stack:
- `L8`  editor product shell and interaction layer
- `L9`  domain authoring suites
- `L10` editor services and extensibility
- `L11` collaboration, review, and production operations

The package targets a large-scale Rust-native editor with the product completeness expected from high-end game tools:
multiple docked panels, scene/world authoring, content browsers, inspectors, viewport overlays, play/simulate/debug, build/release surfaces, assistant integration, and domain suites comparable in breadth to a modern AAA editor.

## Reading order
1. `01_SCOPE.md`
2. `02_EDITOR_STACK.md`
3. `03_EDITOR_ROLE_MAP.md`
4. `04_EDITOR_PRODUCT_MODEL.md`
5. `05_EDITOR_VISUAL_COMPOSITION_MODEL.md`
6. `06_VIEWPORT_AND_NAVIGATION_MODEL.md`
7. `07_SELECTION_AND_INTERACTION_MODEL.md`
8. `08_TOOL_CONTEXT_AND_MODE_MODEL.md`
9. `09_PANEL_AND_VIEW_MODEL.md`
10. `10_COMMAND_PALETTE_AND_SHORTCUT_MODEL.md`
11. `11_CONTENT_AND_ASSET_MODEL.md`
12. `12_OUTLINER_WORLD_BROWSER_MODEL.md`
13. `13_INSPECTOR_AND_DETAILS_MODEL.md`
14. `14_PLAY_SIMULATE_DEBUG_MODEL.md`
15. `15_ASSISTANT_SURFACE_MODEL.md`
16. `16_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md`
17. `17_DOMAIN_SUITE_MODEL.md`
18. `18_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`
19. `19_RESOURCE_DISCIPLINE.md`
20. `20_IMPLEMENTATION_BASELINE.md`
21. `21_TESTING_MODEL.md`
22. `22_ACCEPTANCE_MATRIX.md`
23. `23_EVIDENCE_REGISTRY.md`
24. `24_BUILD_AND_FREEZE_CONDITIONS.md`
25. `25_DOCUMENT_AUTHORITY_ORDER.md`
26. `constitutions/`
27. `levels/`
28. `families/`
29. `99_AUDIT_READINESS_MATRIX.md`

## Canonical rule
- `editor/` defines product surfaces, workflows, and suite composition.
- `editor/` consumes `L6/L6A/L7/L7A` and never weakens them.
- `editor/` owns no hidden shadow-world.
- Equal product data classes must co-reside by lifetime and locality.
