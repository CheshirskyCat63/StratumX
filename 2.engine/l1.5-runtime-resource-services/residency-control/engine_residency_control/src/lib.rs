use bitflags::bitflags;
use engine_core::{EngineCoreError, EngineCoreResult};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidencySet {
    Hot,
    Warm,
    Cold,
    Quarantined,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ResidencyHint: u8 {
        const KEEP_WARM = 0b0001;
        const STREAM_SOON = 0b0010;
        const CRITICAL = 0b0100;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureSignal {
    TrimOptionalWarm,
    DenyOptionalPrefetch,
    DelayOptionalUploads,
    EvictCold,
    DemoteWarm,
    FailIllegal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidencyRecord {
    pub set: ResidencySet,
    pub refcount: u32,
    pub age_ticks: u32,
}

#[derive(Debug, Default)]
pub struct ResidencyController {
    records: BTreeMap<u64, ResidencyRecord>,
}

impl ResidencyController {
    pub fn pin(&mut self, id: u64) {
        let record = self.records.entry(id).or_insert(ResidencyRecord {
            set: ResidencySet::Warm,
            refcount: 0,
            age_ticks: 0,
        });
        record.refcount = record.refcount.saturating_add(1);
        record.set = ResidencySet::Hot;
        record.age_ticks = 0;
    }

    pub fn unpin(&mut self, id: u64) {
        if let Some(record) = self.records.get_mut(&id) {
            record.refcount = record.refcount.saturating_sub(1);
        }
    }

    pub fn step_age(&mut self) {
        for record in self.records.values_mut() {
            record.age_ticks = record.age_ticks.saturating_add(1);
            if record.refcount == 0 {
                if record.age_ticks > 6 && record.set == ResidencySet::Hot {
                    record.set = ResidencySet::Warm;
                } else if record.age_ticks > 12 && record.set == ResidencySet::Warm {
                    record.set = ResidencySet::Cold;
                }
            }
        }
    }

    pub fn quarantine_until_fence(&mut self, id: u64) {
        let record = self.records.entry(id).or_insert(ResidencyRecord {
            set: ResidencySet::Cold,
            refcount: 0,
            age_ticks: 0,
        });
        record.set = ResidencySet::Quarantined;
        record.age_ticks = 0;
    }

    pub fn release_quarantine(
        &mut self,
        id: u64,
        fence_visible: bool,
        grace_ticks: u32,
    ) -> EngineCoreResult<()> {
        let Some(record) = self.records.get_mut(&id) else {
            return Err(EngineCoreError::InvalidDescriptor(
                "residency id not tracked",
            ));
        };
        if record.set != ResidencySet::Quarantined {
            return Ok(());
        }
        if !fence_visible || record.age_ticks < grace_ticks {
            return Err(EngineCoreError::InvariantViolation("quarantined resource may not publish before fence-visible completion and grace window"));
        }
        record.set = ResidencySet::Warm;
        Ok(())
    }

    pub fn apply_pressure(&self) -> PressureSignal {
        let cold = self
            .records
            .values()
            .filter(|r| r.set == ResidencySet::Cold)
            .count();
        if cold > 16 {
            PressureSignal::EvictCold
        } else {
            PressureSignal::TrimOptionalWarm
        }
    }

    pub fn set_of(&self, id: u64) -> Option<ResidencySet> {
        self.records.get(&id).map(|r| r.set)
    }
}
