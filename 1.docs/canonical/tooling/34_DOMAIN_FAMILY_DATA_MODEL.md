# Domain Family Data Model

Domain families are split by data responsibility, not by broad feature mood.
Each family must declare:
- authority-facing minimal truth
- command classes touching that truth
- snapshot classes
- index classes
- derived classes
- artifact classes
- preview classes
- cache classes
- diagnostics classes
- stream classes when applicable
- degradation priorities

Family law:
- families compose planes and sidecars; they do not replace them
- families may define local schemas, but not hidden authority stores
- families with identical locality and lifetime requirements should co-locate arrays or slabs rather than duplicate them in multiple shells
