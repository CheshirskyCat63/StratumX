use engine_runtime::{ApplyRecord, RuntimeKernel, RuntimeLifecycle, RuntimePhase, RuntimeProfile};
use engine_world::{ApplySegment, WorldState};

#[test]
fn runtime_advances_phases_and_applies() {
    let mut runtime = RuntimeKernel::new(RuntimeProfile::Headless20);
    let mut world = WorldState::new();

    runtime.start();
    runtime
        .enqueue_apply(ApplyRecord {
            source_tick: runtime.tick(),
            segment: ApplySegment {
                region_key: (0, 0, 0),
                family_tags: vec![1, 2],
            },
        })
        .expect("enqueue apply");

    runtime.advance_tick(&mut world).expect("advance tick");
    assert_eq!(runtime.phase(), RuntimePhase::Diagnostics);
}

#[test]
fn runtime_lifecycle_transitions() {
    let mut runtime = RuntimeKernel::new(RuntimeProfile::Interactive60);
    assert_eq!(runtime.lifecycle(), RuntimeLifecycle::Stopped);
    runtime.start();
    runtime.pause();
    assert_eq!(runtime.lifecycle(), RuntimeLifecycle::Paused);
    runtime.resume();
    runtime.stop();
    assert_eq!(runtime.lifecycle(), RuntimeLifecycle::Stopped);
}
