# Tool Context and Mode Model

## Context classes
- global editor context
- viewport interaction context
- suite-local domain context
- graph context
- review context
- build/ops context

## Mode classes
- persistent shell mode
- transient modal tool mode
- suite mode
- preview mode
- play/simulate/debug mode

## Laws
- only one focused manipulator context may own high-priority pointer routing at a time
- suite modes must be activation-bounded
- mode switches may invalidate overlays, inspectors, and contextual panels but may not mutate truth implicitly
