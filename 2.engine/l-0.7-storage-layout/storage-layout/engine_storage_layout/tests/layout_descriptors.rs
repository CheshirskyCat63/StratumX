use engine_core::ComponentKind;
use engine_storage_layout::{
    ChunkFreezeLaw, ChunkLayoutDescriptor, ColumnDescriptor, ColumnLayoutDescriptor, LayoutClass,
    LayoutFlags, LayoutValidationError, LocalityClass, LocalityShapeDescriptor,
};
use smallvec::smallvec;

fn component(name: &str, version: u16) -> ComponentKind {
    ComponentKind {
        name: name.to_owned(),
        version,
    }
}

#[test]
fn chunk_descriptor_rejects_unfrozen_plan_requirements() {
    let descriptor = ChunkLayoutDescriptor {
        class: LayoutClass::DenseChunk,
        chunk_capacity: 32,
        partition_count: 2,
        freeze_law: ChunkFreezeLaw {
            signature_frozen: true,
            access_mode_frozen: false,
            invalidation_law_frozen: true,
        },
        flags: LayoutFlags::PUBLIC_IDENTITY_STABLE,
    };

    assert_eq!(
        descriptor.validate(),
        Err(LayoutValidationError::ChunkLayoutRequiresFrozenPlanLaw)
    );
}

#[test]
fn column_descriptor_and_layout_accept_valid_shape() {
    let descriptor = ColumnLayoutDescriptor {
        class: LayoutClass::DenseColumn,
        columns: smallvec![
            ColumnDescriptor {
                component: component("Transform", 1),
                bytes_per_element: 64,
                alignment: 16,
            },
            ColumnDescriptor {
                component: component("Velocity", 1),
                bytes_per_element: 16,
                alignment: 8,
            }
        ],
        flags: LayoutFlags::LOCALITY_CRITICAL,
    };

    assert_eq!(descriptor.validate(), Ok(()));
}

#[test]
fn locality_descriptor_requires_spatial_bucket_for_spatial_class() {
    let descriptor = LocalityShapeDescriptor {
        class: LocalityClass::Spatial,
        cache_line_bytes: 64,
        spatial_bucket_edge: 0,
        partition_count: 1,
        traversal_lane_width: 0,
    };

    assert_eq!(
        descriptor.validate(),
        Err(LayoutValidationError::SpatialBucketMustBeNonZeroForSpatialClass)
    );
}
