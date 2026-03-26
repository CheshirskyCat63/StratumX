use engine_core::{EngineCoreError, EngineCoreResult, Tick};
use engine_ecs::EcsSubstrate;
use engine_world_region::RegionSubstrate;
use serde::{Deserialize, Serialize};

pub const MAX_SEGMENTS_PER_TICK: usize = 256;
pub const MAX_FAMILY_FANOUT_PER_SEGMENT: usize = 8;
pub const MAX_PUBLISH_PASSES: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplySegment {
    pub region_key: (i32, i32, i32),
    pub family_tags: Vec<u16>,
}

#[derive(Debug)]
pub struct WorldState {
    ecs: EcsSubstrate,
    regions: RegionSubstrate,
    tick: Tick,
    epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: Tick,
    pub epoch: u64,
    pub segment_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadModel {
    pub tick: Tick,
    pub epoch: u64,
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            ecs: EcsSubstrate::new(),
            regions: RegionSubstrate::default(),
            tick: Tick(0),
            epoch: 0,
        }
    }

    pub fn ecs_mut(&mut self) -> &mut EcsSubstrate {
        &mut self.ecs
    }

    pub fn regions_mut(&mut self) -> &mut RegionSubstrate {
        &mut self.regions
    }

    pub fn read_model(&self) -> ReadModel {
        ReadModel {
            tick: self.tick,
            epoch: self.epoch,
        }
    }

    pub fn snapshot(&self, segment_count: usize) -> WorldSnapshot {
        WorldSnapshot {
            tick: self.tick,
            epoch: self.epoch,
            segment_count,
        }
    }

    pub fn snapshot_bytes(&self, segment_count: usize) -> EngineCoreResult<Vec<u8>> {
        bincode::serialize(&self.snapshot(segment_count))
            .map_err(|_| EngineCoreError::InvalidDescriptor("snapshot serialization must succeed"))
    }

    pub fn apply(
        &mut self,
        segments: &[ApplySegment],
        publish_passes: usize,
    ) -> EngineCoreResult<()> {
        if segments.len() > MAX_SEGMENTS_PER_TICK {
            return Err(EngineCoreError::InvalidDescriptor(
                "segment count exceeds canonical ceiling",
            ));
        }
        if publish_passes > MAX_PUBLISH_PASSES {
            return Err(EngineCoreError::InvalidDescriptor(
                "publish passes exceed canonical ceiling",
            ));
        }
        for segment in segments {
            if segment.family_tags.len() > MAX_FAMILY_FANOUT_PER_SEGMENT {
                return Err(EngineCoreError::InvalidDescriptor(
                    "family fan-out exceeds canonical ceiling",
                ));
            }
        }

        self.tick = Tick(self.tick.0.saturating_add(1));
        self.epoch = self.epoch.saturating_add(1);
        Ok(())
    }
}
