# Field Contract Rules

Hot-path bridge structs should prefer:
- fixed-width ids
- dense enum classes
- bounded arrays or small vectors
- explicit version fields
- explicit scope fields
- explicit replay class fields

Forbidden in hot-path bridge structs:
- recursive payload graphs
- nested unbounded maps
- editor UI payloads
- raw binary world dumps
- nullable semantic ambiguity where tagged enums are possible
