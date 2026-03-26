use engine_field::{FieldDelta, FieldFamily};

#[test]
fn field_family_tracks_regional_deltas() {
    let mut family = FieldFamily::default();
    family
        .push_delta(FieldDelta {
            region: 1,
            fluid: 0.2,
            thermal: 0.8,
        })
        .unwrap();
    assert_eq!(family.staged_deltas().len(), 1);
}
