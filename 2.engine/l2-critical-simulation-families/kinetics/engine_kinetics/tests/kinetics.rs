use engine_kinetics::{ContactProduct, KineticsFamily, ProjectileLane};

#[test]
fn kinetics_produces_bounded_motion_deltas() {
    let mut family = KineticsFamily::default();
    family
        .record_contact(ContactProduct {
            body_a: 1,
            body_b: 2,
            penetration_mm: 7,
        })
        .unwrap();
    let deltas = family.solve_motion();
    assert_eq!(deltas.len(), 1);
    assert_eq!(deltas[0].body, 1);
}

#[test]
fn far_projectiles_use_far_lane() {
    let family = KineticsFamily::default();
    let p = family.solve_trajectory(10, false);
    assert_eq!(p.lane, ProjectileLane::Far);
}
