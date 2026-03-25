# Panel and View Model

## Mandatory product views
- project launcher / project switcher
- content browser
- outliner
- world browser
- details
- viewport
- diagnostics/output
- task monitor
- assistant dock
- build/release center

## Optional specialist views
- graph editors
- curve editor
- sequencer/timeline
- profiler and budget charts
- validation dashboard
- package dependency view
- source control/change review
- live capture viewer
- playtest session browser

## View law
Views are products. They are not truth stores.
Each view must declare:
- consumed lower-stack data classes
- emitted request classes
- saved UI state classes
- activation and cold-start behavior
