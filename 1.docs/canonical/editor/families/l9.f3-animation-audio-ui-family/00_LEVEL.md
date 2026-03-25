# Family Level

Canonical family: `animation_audio_ui_family`

## Composes
- animation, cinematics, audio, voice, UI, and HUD suites

## Data responsibility
- authority-facing minimal truth: media/UI authority refs only
- snapshot classes: animation/audio/UI snapshots
- index classes: media/UI indices
- derived classes: derived sequencing and layout views
- artifact classes: media/UI artifacts
- preview classes: suite previews
- cache classes: bounded suite caches
- diagnostics classes: animation/audio/UI diagnostics
- degradation priority: `medium`

This family composes editor product surfaces without owning hidden lower-stack truth.
