//! Canonical L-0.7 storage layout substrate.

use bitflags::bitflags;
use engine_core::ComponentKind;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// Canonical workload-oriented layout classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutClass {
    Sparse,
    DenseColumn,
    DenseChunk,
    Paged,
}

bitflags! {
    /// Structural layout flags only.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct LayoutFlags: u8 {
        const PUBLIC_IDENTITY_STABLE = 0b0000_0001;
        const SPARSE_SAFE = 0b0000_0010;
        const LOCALITY_CRITICAL = 0b0000_0100;
    }
}

/// Frozen-plan requirements needed by dense chunk descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkFreezeLaw {
    pub signature_frozen: bool,
    pub access_mode_frozen: bool,
    pub invalidation_law_frozen: bool,
}

impl ChunkFreezeLaw {
    #[must_use]
    pub const fn fully_frozen(&self) -> bool {
        self.signature_frozen && self.access_mode_frozen && self.invalidation_law_frozen
    }
}

/// Chunk-based physical descriptor for hot signatures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkLayoutDescriptor {
    pub class: LayoutClass,
    pub chunk_capacity: u32,
    pub partition_count: u16,
    pub freeze_law: ChunkFreezeLaw,
    pub flags: LayoutFlags,
}

impl ChunkLayoutDescriptor {
    pub const fn validate(&self) -> Result<(), LayoutValidationError> {
        if self.chunk_capacity == 0 {
            return Err(LayoutValidationError::ChunkCapacityMustBeNonZero);
        }

        if self.partition_count == 0 {
            return Err(LayoutValidationError::PartitionCountMustBeNonZero);
        }

        if !matches!(self.class, LayoutClass::DenseChunk) {
            return Err(LayoutValidationError::ChunkLayoutRequiresDenseChunkClass);
        }

        if !self.freeze_law.fully_frozen() {
            return Err(LayoutValidationError::ChunkLayoutRequiresFrozenPlanLaw);
        }

        Ok(())
    }
}

/// Column-level descriptor of one component family column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnDescriptor {
    pub component: ComponentKind,
    pub bytes_per_element: u32,
    pub alignment: u16,
}

impl ColumnDescriptor {
    pub const fn validate(&self) -> Result<(), LayoutValidationError> {
        if self.bytes_per_element == 0 {
            return Err(LayoutValidationError::ColumnElementSizeMustBeNonZero);
        }

        if self.alignment == 0 {
            return Err(LayoutValidationError::ColumnAlignmentMustBeNonZero);
        }

        Ok(())
    }
}

/// Column-oriented layout descriptor for component-aligned traversal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnLayoutDescriptor {
    pub class: LayoutClass,
    pub columns: SmallVec<[ColumnDescriptor; 8]>,
    pub flags: LayoutFlags,
}

impl ColumnLayoutDescriptor {
    pub fn validate(&self) -> Result<(), LayoutValidationError> {
        if !matches!(self.class, LayoutClass::DenseColumn) {
            return Err(LayoutValidationError::ColumnLayoutRequiresDenseColumnClass);
        }

        if self.columns.is_empty() {
            return Err(LayoutValidationError::ColumnLayoutRequiresColumns);
        }

        for column in &self.columns {
            column.validate()?;
        }

        for (idx, left) in self.columns.iter().enumerate() {
            let duplicate_found = self.columns[idx + 1..].iter().any(|right| {
                left.component.name == right.component.name
                    && left.component.version == right.component.version
            });
            if duplicate_found {
                return Err(LayoutValidationError::DuplicateComponentColumn {
                    name: left.component.name.clone(),
                    version: left.component.version,
                });
            }
        }

        Ok(())
    }
}

/// Locality classes that shape layout choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalityClass {
    Cache,
    Spatial,
    Partition,
    TraversalLane,
}

/// Locality-oriented shape descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityShapeDescriptor {
    pub class: LocalityClass,
    pub cache_line_bytes: u16,
    pub spatial_bucket_edge: u16,
    pub partition_count: u16,
    pub traversal_lane_width: u16,
}

impl LocalityShapeDescriptor {
    pub const fn validate(&self) -> Result<(), LayoutValidationError> {
        if self.cache_line_bytes == 0 {
            return Err(LayoutValidationError::CacheLineBytesMustBeNonZero);
        }

        if self.partition_count == 0 {
            return Err(LayoutValidationError::PartitionCountMustBeNonZero);
        }

        if matches!(self.class, LocalityClass::Spatial) && self.spatial_bucket_edge == 0 {
            return Err(LayoutValidationError::SpatialBucketMustBeNonZeroForSpatialClass);
        }

        if matches!(self.class, LocalityClass::TraversalLane) && self.traversal_lane_width == 0 {
            return Err(LayoutValidationError::TraversalLaneWidthMustBeNonZeroForLaneClass);
        }

        Ok(())
    }
}

/// Validation errors for storage layout descriptors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutValidationError {
    ChunkCapacityMustBeNonZero,
    PartitionCountMustBeNonZero,
    ChunkLayoutRequiresDenseChunkClass,
    ChunkLayoutRequiresFrozenPlanLaw,
    ColumnElementSizeMustBeNonZero,
    ColumnAlignmentMustBeNonZero,
    ColumnLayoutRequiresDenseColumnClass,
    ColumnLayoutRequiresColumns,
    DuplicateComponentColumn { name: String, version: u16 },
    CacheLineBytesMustBeNonZero,
    SpatialBucketMustBeNonZeroForSpatialClass,
    TraversalLaneWidthMustBeNonZeroForLaneClass,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn component(name: &str, version: u16) -> ComponentKind {
        ComponentKind {
            name: name.to_owned(),
            version,
        }
    }

    #[test]
    fn chunk_layout_requires_dense_chunk_and_frozen_law() {
        let descriptor = ChunkLayoutDescriptor {
            class: LayoutClass::DenseChunk,
            chunk_capacity: 64,
            partition_count: 4,
            freeze_law: ChunkFreezeLaw {
                signature_frozen: true,
                access_mode_frozen: true,
                invalidation_law_frozen: true,
            },
            flags: LayoutFlags::PUBLIC_IDENTITY_STABLE | LayoutFlags::LOCALITY_CRITICAL,
        };

        assert_eq!(descriptor.validate(), Ok(()));
    }

    #[test]
    fn column_layout_rejects_duplicate_component_columns() {
        let descriptor = ColumnLayoutDescriptor {
            class: LayoutClass::DenseColumn,
            columns: SmallVec::from_vec(vec![
                ColumnDescriptor {
                    component: component("Transform", 1),
                    bytes_per_element: 64,
                    alignment: 16,
                },
                ColumnDescriptor {
                    component: component("Transform", 1),
                    bytes_per_element: 64,
                    alignment: 16,
                },
            ]),
            flags: LayoutFlags::SPARSE_SAFE,
        };

        assert_eq!(
            descriptor.validate(),
            Err(LayoutValidationError::DuplicateComponentColumn {
                name: "Transform".to_owned(),
                version: 1,
            })
        );
    }

    #[test]
    fn locality_model_requires_lane_width_for_traversal_lane_class() {
        let descriptor = LocalityShapeDescriptor {
            class: LocalityClass::TraversalLane,
            cache_line_bytes: 64,
            spatial_bucket_edge: 0,
            partition_count: 2,
            traversal_lane_width: 0,
        };

        assert_eq!(
            descriptor.validate(),
            Err(LayoutValidationError::TraversalLaneWidthMustBeNonZeroForLaneClass)
        );
    }
}
