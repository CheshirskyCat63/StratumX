use engine_acoustics::propagate;

#[test]
fn acoustics_propagation_attenuates_with_distance() {
    let near = propagate(10.0);
    let far = propagate(100.0);
    assert!(far.attenuation_db < near.attenuation_db);
}
