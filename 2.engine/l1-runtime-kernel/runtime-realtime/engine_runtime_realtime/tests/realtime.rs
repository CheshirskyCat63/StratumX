use engine_runtime_realtime::{
    PresentationPolicy, RealtimeRole, RealtimeRuntime, REALTIME_FRAME_HZ,
};
use engine_world::WorldState;

#[test]
fn realtime_profile_produces_presentable_frame() {
    let mut rt = RealtimeRuntime::new(
        RealtimeRole::InteractiveClient,
        PresentationPolicy::LowLatency,
    );
    let mut world = WorldState::new();
    rt.start();
    let out = rt.step_frame(&mut world).expect("step frame");
    assert_eq!(out.cadence_hz, REALTIME_FRAME_HZ);
    assert!(out.presentable);
    assert_eq!(out.frame, 1);
}
