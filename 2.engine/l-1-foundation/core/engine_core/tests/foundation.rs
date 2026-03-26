use engine_core::{
    EngineCoreError, EngineDescriptor, EntityId, FeatureProfile, StaticDescriptor, Tick,
    ACTIVE_PROFILE,
};

#[test]
fn descriptor_validation_rejects_empty_labels() {
    let descriptor = StaticDescriptor {
        label: String::new(),
    };

    let error = descriptor
        .validate()
        .expect_err("empty descriptor must fail");
    assert_eq!(
        error,
        EngineCoreError::InvalidDescriptor("label must not be empty")
    );
}

#[test]
fn foundational_types_round_trip_identity_values() {
    let entity = EntityId(42);
    let tick = Tick(7);

    assert_eq!(entity.0, 42);
    assert_eq!(tick.0, 7);
}

#[test]
fn active_profile_is_a_legal_value() {
    assert!(matches!(
        ACTIVE_PROFILE,
        FeatureProfile::Minimal | FeatureProfile::Headless | FeatureProfile::Realtime
    ));
}
