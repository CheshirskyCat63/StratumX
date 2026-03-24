# Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| compat_version_set_id | CompatVersionSetId | required | stable identity of one compatibility version set | must remain stable for the declared version tuple |
| bridge_protocol_version | SemVer | required | version of the L5 bridge protocol | must follow semantic versioning |
| l4_contract_version | SemVer | required | version of the public L4 contract that this bridge targets | must follow semantic versioning |
| l6_min_supported_version | SemVer | required | lowest compatible L6 contract version | must follow semantic versioning |
| stack_version_ref | StackVersionRef | required | links the version set to the package stack version | must match `STACK_VERSION` |
