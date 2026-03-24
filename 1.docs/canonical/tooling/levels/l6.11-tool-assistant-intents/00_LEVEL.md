# L6.11 Level

Canonical layer: `tool_assistant_intents`

Exists to own exactly one tooling role: assistant workflow intents.
Core data classes: AssistantPromptIntent, AssistantBootstrapIntent, AssistantRepairIntent, AssistantRefactorIntent, AssistantBuildAssistIntent.
It explicitly does not own: generation execution, direct project mutation, direct engine access, hidden planning state.
It exists to keep its adjacent layers from collapsing into one mixed layer.
