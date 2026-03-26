use bitflags::bitflags;
use engine_core::Tick;
use engine_world_spatial::{ChunkAddress, RegionAddress};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const CHUNK_EDGE_METERS: u32 = 32;
pub const VERTICAL_SLAB_METERS: u32 = 16;
pub const SAME_TICK_HALO_WIDTH: u8 = 1;
pub const MAX_FIELD_SOLVE_HALO_WIDTH: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionVersion {
    pub epoch: u64,
    pub last_dirty_tick: Tick,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionDescriptor {
    pub address: RegionAddress,
    pub version: RegionVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkDescriptor {
    pub address: ChunkAddress,
    pub region: RegionAddress,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DirtyFlags: u8 {
        const GEOMETRY = 0b0001;
        const ECS = 0b0010;
        const MATERIAL = 0b0100;
    }
}

#[derive(Debug, Default)]
pub struct RegionSubstrate {
    regions: BTreeMap<RegionAddress, RegionDescriptor>,
    dirty_chunks: BTreeMap<RegionAddress, BTreeMap<ChunkAddress, DirtyFlags>>,
}

impl RegionSubstrate {
    pub fn register_region(&mut self, address: RegionAddress, tick: Tick) {
        self.regions.entry(address).or_insert(RegionDescriptor {
            address,
            version: RegionVersion {
                epoch: 0,
                last_dirty_tick: tick,
            },
        });
    }

    pub fn chunk_descriptor(&self, address: ChunkAddress) -> ChunkDescriptor {
        ChunkDescriptor {
            region: address.region,
            address,
        }
    }

    pub fn mark_dirty(&mut self, chunk: ChunkAddress, flags: DirtyFlags, tick: Tick) {
        self.register_region(chunk.region, tick);
        self.dirty_chunks
            .entry(chunk.region)
            .or_default()
            .entry(chunk)
            .and_modify(|f| *f |= flags)
            .or_insert(flags);

        if let Some(region) = self.regions.get_mut(&chunk.region) {
            region.version.epoch = region.version.epoch.saturating_add(1);
            region.version.last_dirty_tick = tick;
        }
    }

    pub fn dirty_chunks(&self, region: RegionAddress) -> BTreeSet<ChunkAddress> {
        self.dirty_chunks
            .get(&region)
            .map(|v| v.keys().copied().collect())
            .unwrap_or_default()
    }

    pub fn region_version(&self, region: RegionAddress) -> Option<RegionVersion> {
        self.regions.get(&region).map(|r| r.version)
    }
}
