use engine_handle::{EntityHandle, HandleGeneration, HandleSlot, StableHandle};
use engine_identity::{ComponentKind, EntityId};
use engine_storage_access::LayoutClass;
use engine_storage_mutation::{
    ApplyPayload, ChangeSet, DeferredWrite, FamilyTag, MutationBuffer, MutationFlags,
    MutationStage, MutationValidationError, RegionTag,
};
use smallvec::SmallVec;

fn handle(entity: u64, slot: u32, generation: u32) -> EntityHandle {
    EntityHandle::new(
        EntityId(entity),
        StableHandle::new(HandleSlot(slot), HandleGeneration(generation)),
    )
}

fn write(stage: MutationStage, bytes: &[u8], flags: MutationFlags) -> DeferredWrite {
    DeferredWrite {
        target: handle(100, 8, 1),
        component: ComponentKind {
            name: "Health".to_owned(),
            version: 1,
        },
        bytes: SmallVec::from_slice(bytes),
        access_class: LayoutClass::DenseChunk,
        stage,
        flags,
    }
}

#[test]
fn buffer_staging_and_change_set_packaging() {
    let mut buffer = MutationBuffer::new();
    buffer
        .stage(write(MutationStage::Staged, &[4, 2], MutationFlags::DATA))
        .unwrap();

    let set = buffer
        .into_change_set(17, RegionTag(3), FamilyTag("physics".to_owned()))
        .unwrap();

    assert_eq!(set.tick, 17);
    assert_eq!(set.region, RegionTag(3));
    assert_eq!(set.writes.len(), 1);
}

#[test]
fn deferred_write_enforces_non_applied_semantics() {
    let staged = write(MutationStage::Packaged, &[9], MutationFlags::STRUCTURAL);
    assert_eq!(staged.validate(), Ok(()));

    let applied = write(MutationStage::Applied, &[9], MutationFlags::STRUCTURAL);
    assert_eq!(
        applied.validate(),
        Err(MutationValidationError::AppliedStateForbiddenInMutationCrate)
    );
}

#[test]
fn changeset_rejects_applied_entries() {
    let writes = SmallVec::from_vec(vec![write(
        MutationStage::Applied,
        &[1, 2, 3],
        MutationFlags::DATA,
    )]);

    assert_eq!(
        ChangeSet::new(1, RegionTag(1), FamilyTag("ai".to_owned()), writes),
        Err(MutationValidationError::AppliedStateForbiddenInMutationCrate)
    );
}

#[test]
fn apply_payload_validity_requires_non_empty_non_applied_entries() {
    let payload = ApplyPayload {
        tick: 20,
        region: RegionTag(2),
        family: FamilyTag("agents".to_owned()),
        writes: SmallVec::from_vec(vec![write(
            MutationStage::Packaged,
            &[7],
            MutationFlags::IDEMPOTENT,
        )]),
    };
    assert_eq!(payload.validate(), Ok(()));

    let invalid = ApplyPayload {
        tick: 20,
        region: RegionTag(2),
        family: FamilyTag("agents".to_owned()),
        writes: SmallVec::from_vec(vec![write(
            MutationStage::Applied,
            &[7],
            MutationFlags::IDEMPOTENT,
        )]),
    };
    assert_eq!(
        invalid.validate(),
        Err(MutationValidationError::AppliedStateForbiddenInMutationCrate)
    );
}
