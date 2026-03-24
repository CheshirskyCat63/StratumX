# L6.10 Level

Canonical layer: `tool_release_intents`

Exists to own exactly one tooling role: release workflow intents.
Core data classes: BuildIntentRef, PackageIntentRef, ExportIntentRef, TestRunIntentRef.
It explicitly does not own: build execution, package execution, release truth, deploy runtime.
It exists to keep its adjacent layers from collapsing into one mixed layer.
