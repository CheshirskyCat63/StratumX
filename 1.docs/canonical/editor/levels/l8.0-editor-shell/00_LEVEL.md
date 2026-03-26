# editor_shell Level

Canonical layer: `editor_shell`
Activation class: `warm-shell`.

## Role
editor shell is the canonical product/service surface for this level.
It exists above the lower-stack authority layers and may only host views, contexts, services, jobs, or requests appropriate to its role.

## Owns
- menu bar, main toolbar, tab host, status strip, shell routing, workspace frame, and shell-visible command hubs

## Consumes
- workspace layout state, lower-stack status signals, command palette requests, assistant/diagnostics/build surface presence

## Emits
- shell actions, view host activation, workspace change requests, shell-level product intents

## Data classes
- view or service-local state appropriate to this layer
- activation and visibility state
- request and result envelopes appropriate to this layer

## Concurrency law
- focused UI routing remains single-writer where applicable
- background work may exist only when bounded and visible to diagnostics/budget surfaces

## Never owns
- lower-stack truth, lower-stack transactions, project/game truth
