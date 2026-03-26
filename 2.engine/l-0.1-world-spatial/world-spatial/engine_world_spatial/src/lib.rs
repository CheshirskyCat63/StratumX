use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RegionAddress {
    pub x: i32,
    pub y: i32,
    pub slab_z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChunkAddress {
    pub region: RegionAddress,
    pub chunk_x: i32,
    pub chunk_y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WorldCoordinate {
    pub meters: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialRelation {
    Adjacent,
    Contained,
    Disjoint,
}

pub const CHUNK_EDGE_METERS: f32 = 32.0;
pub const VERTICAL_SLAB_METERS: f32 = 16.0;

pub fn address_for_world_coordinate(coord: WorldCoordinate) -> ChunkAddress {
    let chunk_x = (coord.meters.x / CHUNK_EDGE_METERS).floor() as i32;
    let chunk_y = (coord.meters.y / CHUNK_EDGE_METERS).floor() as i32;
    let slab_z = (coord.meters.z / VERTICAL_SLAB_METERS).floor() as i32;

    ChunkAddress {
        region: RegionAddress {
            x: chunk_x.div_euclid(32),
            y: chunk_y.div_euclid(32),
            slab_z,
        },
        chunk_x,
        chunk_y,
    }
}

pub fn compose_transform(parent: Transform, local: Transform) -> Transform {
    Transform {
        translation: parent.translation + local.translation,
        rotation: parent.rotation * local.rotation,
        scale: parent.scale * local.scale,
    }
}

pub fn classify_relation(a: ChunkAddress, b: ChunkAddress) -> SpatialRelation {
    if a == b {
        return SpatialRelation::Contained;
    }

    let dx = (a.chunk_x - b.chunk_x).abs();
    let dy = (a.chunk_y - b.chunk_y).abs();
    if dx <= 1 && dy <= 1 && a.region.slab_z == b.region.slab_z {
        SpatialRelation::Adjacent
    } else {
        SpatialRelation::Disjoint
    }
}
