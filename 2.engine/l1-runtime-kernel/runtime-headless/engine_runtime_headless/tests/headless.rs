use engine_runtime_headless::{HeadlessRole, HeadlessRuntime, HEADLESS_CADENCE_HZ};
use engine_world::WorldState;

#[test]
fn headless_profile_steps_at_configured_cadence() {
    let mut rt = HeadlessRuntime::new(HeadlessRole::DedicatedServer);
    let mut world = WorldState::new();
    rt.start();
    let outputs = rt.step(&mut world).expect("step");
    assert_eq!(outputs.cadence_hz, HEADLESS_CADENCE_HZ);
    assert_eq!(outputs.tick, 1);
}
