use engine_memory_control::{AllocatorClass, MemoryController, PressureStep, ProfileClass};

#[test]
fn memory_enforces_profile_ceiling() {
    let mut memory = MemoryController::new(ProfileClass::Headless20);
    let ok = memory.allocate(AllocatorClass::FrameScratch, 8 * 1024 * 1024);
    assert!(ok.is_ok());
    let err = memory.allocate(AllocatorClass::FrameScratch, 1);
    assert!(err.is_err());
    assert_eq!(memory.pressure_step(), PressureStep::CompactPools);
}

#[test]
fn memory_release_reduces_usage() {
    let mut memory = MemoryController::new(ProfileClass::Interactive60);
    memory
        .allocate(AllocatorClass::TickScratch, 1024)
        .expect("allocate");
    memory.release(AllocatorClass::TickScratch, 256);
    assert_eq!(memory.usage_of(AllocatorClass::TickScratch), 768);
}
