# animation_audio_ui_family Family

Canonical family: `animation_audio_ui_family`.

## Role
This family groups related editor product surfaces or services that share data locality, activation regime, and request/view discipline.

## Owns
- animation, cinematics, audio/voice, UI/HUD, and quest/event/logic suites

## Consumes
- animation/audio/ui/logic projections, graph service hooks

## Emits
- authoring requests for those suites

## Family law
Members of this family cohere by data type, lifetime, and invalidation regime.
They may share indices, projections, and activation policy where legal.

## Never owns
- suite truth
