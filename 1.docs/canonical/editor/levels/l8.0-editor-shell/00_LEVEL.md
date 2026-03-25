# editor_shell Level

Canonical layer: `editor_shell`
Activation class: `warm-shell`.

## Owns
- menu bar, main toolbar, tab host, status bar, shell routing, and project/workspace frame

## Consumes
- workspace layout state, command palette requests, lower-stack status signals

## Emits
- shell actions, view host activation, and workspace change requests

## Never owns
- lower-stack truth
