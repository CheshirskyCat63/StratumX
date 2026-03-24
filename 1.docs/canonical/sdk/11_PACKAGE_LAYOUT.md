# Package Layout

## Root package layout
```text
docs/canonical/sdk/
├── 00_INDEX.md
├── 01_SCOPE.md
├── 02_CANONICAL_STACK.md
├── 03_ROLE_MAP.md
├── 04_LIBRARY_BASELINE.md
├── 05_DEPENDENCY_MODEL.md
├── 06_COMMUNICATION_MODEL.md
├── 07_THREADING_MODEL.md
├── 08_EXECUTION_FLOW.md
├── 09_GLOSSARY.md
├── 10_DOCUMENT_RULES.md
├── 11_PACKAGE_LAYOUT.md
├── 12_BOUNDARY_PRESERVATION_MATRIX.md
├── 13_TYPOLOGY_SYSTEM.md
├── 14_ROLE_CLASS_SEPARATION_MATRIX.md
├── 15_FIELD_CONTRACT_RULES.md
├── 16_BOUNDARY_AUTHORITY.md
├── 17_REF_SUBTYPES.md
├── 18_RESULT_ARTIFACT_VERDICT_SEPARATION.md
├── 19_TASK_GRAPH_EDGE_LAW.md
├── 20_ASSISTANT_SEMANTIC_SEPARATION.md
├── 21_GENERATION_TRIANGLE_SEPARATION.md
├── 22_L4_SYNCHRONIZATION_MODEL.md
├── 23_BUILD_AND_FREEZE_CONDITIONS.md
├── 24_TESTING_MODEL.md
├── 25_IMPLEMENTATION_HANDOFF.md
├── 26_SHARED_TYPE_REGISTRY.md
├── 27_ACCEPTANCE_MATRIX.md
├── 28_PACKET_AND_OBSERVATION_NAMING.md
├── 29_DOCUMENT_AUTHORITY_ORDER.md
├── 30_EVIDENCE_REGISTRY.md
├── 99_AUDIT_READINESS_MATRIX.md
├── STACK_VERSION
├── constitutions/
│   ├── 00_INDEX.md
│   ├── STRATUMX_SDK_L5_ASSISTANT_AND_GENERATION_CONSTITUTION.md
│   ├── STRATUMX_SDK_L5_CONCURRENCY_CONSTITUTION.md
│   ├── STRATUMX_SDK_L5_CONSTITUTION.md
│   ├── STRATUMX_SDK_L5_DEPENDENCY_CONSTITUTION.md
│   ├── STRATUMX_SDK_L5_FIELD_SEMANTICS_CONSTITUTION.md
│   ├── STRATUMX_SDK_L5_NON_INTERFERENCE_LAW.md
│   ├── STRATUMX_SDK_L5_PACKET_AND_ENVELOPE_LAW.md
│   ├── STRATUMX_SDK_L5_RESULT_ARTIFACT_VERDICT_SEPARATION.md
│   ├── STRATUMX_SDK_L5_SEMANTIC_SEALING_CONSTITUTION.md
│   ├── STRATUMX_SDK_L5_SHARED_TYPE_REGISTRY_CONSTITUTION.md
│   ├── STRATUMX_SDK_L5_STRUCTURAL_STRATA_CONSTITUTION.md
│   ├── STRATUMX_SDK_L5_TASK_GRAPH_CONSTITUTION.md
│   ├── STRATUMX_SDK_L5_TYPE_SYSTEM_CONSTITUTION.md
├── evidence/
│   ├── root/
│   │   ├── root_traceability_log_v10.md
│   │   ├── root_traceability_log_v11.md
│   │   ├── root_traceability_log_v12.md
│   │   ├── root_traceability_log_v9.md
│   └── layers/
│       ├── layer_traceability_log_v10.md
│       ├── layer_traceability_log_v11.md
│       ├── layer_traceability_log_v12.md
│       ├── layer_traceability_log_v9.md
└── levels/
    ├── l5.0-link-ingress-packets/
    ├── l5.1-link-ingress-controls/
    ├── l5.10-engine-session-handles/
    ├── l5.11-engine-object-handles/
    ├── l5.12-engine-runtime-handles/
    ├── l5.13-engine-identity-refs/
    ├── l5.14-engine-state-refs/
    ├── l5.2-link-egress-observations/
    ├── l5.3-link-egress-metrics/
    ├── l5.4-compat-versions/
    ├── l5.5-compat-capabilities/
    ├── l5.6-compat-profiles/
    ├── l5.7-compat-verdicts/
    ├── l5.8-transport-policies/
    └── l5.9-legality-gates/
```

## Root document truth rule
- this document is the canonical photograph of the root package and must match the real package exactly
- no root document is optional unless this file marks it optional explicitly
- if a root document exists in the package and is absent here, this file is stale and must be fixed before freeze

## Root document set
- core root docs: `00` through `30` plus `99` and `STACK_VERSION`
- constitutions: summary constitutions and cross-cutting laws only
- levels: one level folder per canonical layer

## Level layout
Every level folder contains `00_LEVEL.md` and one layer folder.

## Layer folder layout
Every layer folder contains:
- `00_LAYER.md`
- `10_LIBRARIES.md`
- `20_DEPENDENCIES.md`
- `30_COMMUNICATION.md`
- `31_THREADING.md`
- `32_BOUNDARY_PRESERVATION.md`
- `40_FIELDS.md`

## Forbidden drift
- do not treat omitted root docs as optional by implication
- do not let constitutions or level docs introduce files not reflected here
- do not keep this document as a historical snapshot; it must always match the current package
