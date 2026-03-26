use engine_core::{EngineCoreError, EngineCoreResult};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllocatorClass {
    FrameScratch,
    TickScratch,
    CellScratch,
    SessionPersistent,
    StreamingResident,
    StagingBacked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureStep {
    TrimOptionalCaches,
    CompactPools,
    DenyOptionalOperations,
    EscalateResidencyDemotion,
    FailRequests,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileClass {
    Interactive60,
    ListenHost60,
    Headless20,
}

#[derive(Debug)]
pub struct MemoryController {
    profile: ProfileClass,
    usage: BTreeMap<AllocatorClass, usize>,
    pressure: PressureStep,
}

impl MemoryController {
    pub fn new(profile: ProfileClass) -> Self {
        let mut usage = BTreeMap::new();
        for class in [
            AllocatorClass::FrameScratch,
            AllocatorClass::TickScratch,
            AllocatorClass::CellScratch,
            AllocatorClass::SessionPersistent,
            AllocatorClass::StreamingResident,
            AllocatorClass::StagingBacked,
        ] {
            usage.insert(class, 0);
        }
        Self {
            profile,
            usage,
            pressure: PressureStep::TrimOptionalCaches,
        }
    }

    pub fn allocate(&mut self, class: AllocatorClass, bytes: usize) -> EngineCoreResult<()> {
        let limit = self.ceiling(class);
        let current = *self.usage.get(&class).unwrap_or(&0);
        let next = current.saturating_add(bytes);
        if next > limit {
            self.advance_pressure();
            return Err(EngineCoreError::InvalidDescriptor(
                "allocator class ceiling exceeded",
            ));
        }
        self.usage.insert(class, next);
        Ok(())
    }

    pub fn release(&mut self, class: AllocatorClass, bytes: usize) {
        let current = *self.usage.get(&class).unwrap_or(&0);
        self.usage.insert(class, current.saturating_sub(bytes));
    }

    pub fn pressure_step(&self) -> PressureStep {
        self.pressure
    }

    pub fn usage_of(&self, class: AllocatorClass) -> usize {
        *self.usage.get(&class).unwrap_or(&0)
    }

    fn advance_pressure(&mut self) {
        self.pressure = match self.pressure {
            PressureStep::TrimOptionalCaches => PressureStep::CompactPools,
            PressureStep::CompactPools => PressureStep::DenyOptionalOperations,
            PressureStep::DenyOptionalOperations => PressureStep::EscalateResidencyDemotion,
            PressureStep::EscalateResidencyDemotion => PressureStep::FailRequests,
            PressureStep::FailRequests => PressureStep::FailRequests,
        }
    }

    fn ceiling(&self, class: AllocatorClass) -> usize {
        const MIB: usize = 1024 * 1024;
        match (self.profile, class) {
            (ProfileClass::Interactive60, AllocatorClass::FrameScratch) => 64 * MIB,
            (ProfileClass::ListenHost60, AllocatorClass::FrameScratch) => 64 * MIB,
            (ProfileClass::Headless20, AllocatorClass::FrameScratch) => 8 * MIB,
            (ProfileClass::Interactive60, AllocatorClass::TickScratch) => 96 * MIB,
            (ProfileClass::ListenHost60, AllocatorClass::TickScratch) => 128 * MIB,
            (ProfileClass::Headless20, AllocatorClass::TickScratch) => 128 * MIB,
            (ProfileClass::Interactive60, AllocatorClass::CellScratch) => 128 * MIB,
            (ProfileClass::ListenHost60, AllocatorClass::CellScratch) => 160 * MIB,
            (ProfileClass::Headless20, AllocatorClass::CellScratch) => 192 * MIB,
            (ProfileClass::Interactive60, AllocatorClass::SessionPersistent) => 384 * MIB,
            (ProfileClass::ListenHost60, AllocatorClass::SessionPersistent) => 512 * MIB,
            (ProfileClass::Headless20, AllocatorClass::SessionPersistent) => 512 * MIB,
            (ProfileClass::Interactive60, AllocatorClass::StreamingResident) => 1280 * MIB,
            (ProfileClass::ListenHost60, AllocatorClass::StreamingResident) => 1536 * MIB,
            (ProfileClass::Headless20, AllocatorClass::StreamingResident) => 1152 * MIB,
            (ProfileClass::Interactive60, AllocatorClass::StagingBacked) => 192 * MIB,
            (ProfileClass::ListenHost60, AllocatorClass::StagingBacked) => 256 * MIB,
            (ProfileClass::Headless20, AllocatorClass::StagingBacked) => 64 * MIB,
        }
    }
}
