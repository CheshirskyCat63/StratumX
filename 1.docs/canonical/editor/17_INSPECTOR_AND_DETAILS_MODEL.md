# Inspector and Details Model

## Responsibilities
- typed details for focused objects and selections
- batch editing where legal
- validation hints and provenance display
- preview-safe value staging
- assistant and diagnostics context handoff

## Laws
- inspector owns presentation and staging only
- staged edits must lower into legal lower-stack requests
- hidden persistent side stores are forbidden
