# Scope

This package freezes the L5 SDK bridge as the only legal typed connection between engine L4 and the upper tools stack.

It defines:
- write-side bridge envelopes
- read-side bridge envelopes
- compatibility and legality surfaces
- transport policy surfaces
- opaque handle and ref classes
- artifact ref publication toward tools
- pressure-safe and replay-safe bridge laws

It does not define:
- engine internals
- editor truth
- assistant runtime truth
- studio orchestration truth
- planning truth
- generated artifact ownership
