        # Fields

        - `inspection_view_id: InspectionViewId` required, unique.
- `source_ref: TypedSourceRef` required.
- `property_rows: PropertyRow[]` required, many.
- `schema_projection_ref: SchemaProjectionRef` optional.
- `diagnostic_overlay_refs: DiagnosticOverlayRef[]` optional, many.

        Cardinality, ownership, and invariants are part of canon.
