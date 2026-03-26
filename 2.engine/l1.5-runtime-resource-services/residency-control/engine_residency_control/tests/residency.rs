use engine_residency_control::{ResidencyController, ResidencySet};

#[test]
fn residency_demotes_with_hysteresis() {
    let mut controller = ResidencyController::default();
    controller.pin(7);
    controller.unpin(7);
    for _ in 0..13 {
        controller.step_age();
    }
    assert_eq!(controller.set_of(7), Some(ResidencySet::Cold));
}

#[test]
fn quarantine_requires_fence_and_grace() {
    let mut controller = ResidencyController::default();
    controller.quarantine_until_fence(9);
    controller.step_age();
    assert!(controller.release_quarantine(9, false, 1).is_err());
    assert!(controller.release_quarantine(9, true, 1).is_ok());
}
