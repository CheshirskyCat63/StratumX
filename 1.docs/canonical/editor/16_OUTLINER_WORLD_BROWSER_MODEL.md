# Outliner and World Browser Model

## Responsibilities
- hierarchy projection
- world partition and region views
- visibility and lock toggles
- grouping, parenting, and folder views
- search, filter, and scope slicing

## Laws
- hierarchy views are projections of lower-stack state
- grouping actions become legal lower-stack requests
- world browser hosts scale to very large scenes through activation-bounded indexing and partial projection
