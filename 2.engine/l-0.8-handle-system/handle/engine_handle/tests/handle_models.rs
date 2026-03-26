use engine_handle::{
    is_stale_generation, validate_handle, HandleGeneration, HandleSlot, HandleValidation,
    InvalidationMarker, StableHandle,
};

#[test]
fn stale_detection_helper_rejects_generation_mismatch() {
    assert!(is_stale_generation(
        HandleGeneration(1),
        HandleGeneration(2)
    ));
    assert!(!is_stale_generation(
        HandleGeneration(2),
        HandleGeneration(2)
    ));
}

#[test]
fn validation_reports_stale_observed_and_expected_generations() {
    let handle = StableHandle::new(HandleSlot(0), HandleGeneration(8));
    let marker = InvalidationMarker::new(HandleGeneration(12));

    assert_eq!(
        validate_handle(handle, marker),
        HandleValidation::Stale {
            expected: HandleGeneration(12),
            observed: HandleGeneration(8)
        }
    );
}
