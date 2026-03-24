# Glossary

- **fact**: compact owned state for one lane.
- **ref**: compact typed reference to an external or separately-owned object.
- **request**: typed ask for work; not execution itself.
- **intent**: user- or workflow-originating requested action, still above execution.
- **event**: typed published observation from below or from a task source.
- **projection**: derived read-side model built from facts, refs, events, or results.
- **result ref**: typed reference to the outcome of completed work.
- **activation rule**: declarative condition under which a family or lane may wake.
- **activation state**: fact recording actual activation.
- **panel ref**: typed attachment point for a panel-level UI surface.
- **view ref**: typed attachment point for a view-level UI surface.
- **family**: composition of multiple lanes serving one tooling domain.
- **dormant**: declared but not active and not executing.
- **disposed**: activation and temporary state released.
- **fan-in**: many inputs merged into one typed projection or request.
- **fan-out**: one event source consumed by many readers.
