# L6.F0 Level

Canonical layer: `editor_shell_family`

Exists to own exactly one tooling role: shell composition.
Core data classes: shell frame, command bar, panel docking, view host, theme tokens.
It explicitly does not own: content browsing, diagnostics truth, build execution, project truth.
It exists to keep its adjacent layers from collapsing into one mixed layer.
