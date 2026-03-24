# StratumX SDK L5 Field Semantics Constitution

Field contracts are closed and type-led.

- ids are stable only within the scope named by the layer contract
- enums and flags must come from the shared registry
- handles and refs are opaque upward
- optional reason refs and anchor refs are references only and never payload bodies
