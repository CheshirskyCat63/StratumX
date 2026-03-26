use bitflags::bitflags;
use engine_core::{ComponentTypeId, EngineCoreError, EngineCoreResult};
use engine_storage_access::WriteWindow;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdempotenceClass {
    Idempotent,
    NonIdempotent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FamilyTag(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionTag(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredWrite {
    pub component: ComponentTypeId,
    pub bytes: SmallVec<[u8; 32]>,
    pub idempotence: IdempotenceClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationBuffer {
    writes: SmallVec<[DeferredWrite; 16]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSet {
    pub structural: SmallVec<[ComponentTypeId; 8]>,
    pub writes: SmallVec<[DeferredWrite; 16]>,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ApplyFlags: u8 {
        const SEGMENTED = 0b0001;
        const ALLOW_TOMBSTONE_COMPACTION = 0b0010;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyPayload {
    pub family_tag: FamilyTag,
    pub region_tag: RegionTag,
    pub batch_order: u64,
    pub flags: ApplyFlags,
    pub change_set: ChangeSet,
}

impl Default for MutationBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl MutationBuffer {
    pub fn new() -> Self {
        Self {
            writes: SmallVec::new(),
        }
    }

    pub fn stage_write(&mut self, write: DeferredWrite) {
        if let Some(existing) = self.writes.iter_mut().find(|w| {
            w.component == write.component && w.idempotence == IdempotenceClass::Idempotent
        }) {
            *existing = write;
            return;
        }
        self.writes.push(write);
    }

    pub fn into_change_set(self, structural: SmallVec<[ComponentTypeId; 8]>) -> ChangeSet {
        ChangeSet {
            structural,
            writes: self.writes,
        }
    }
}

pub fn queue_deferred_writes(
    window: &WriteWindow,
    buffer: MutationBuffer,
) -> EngineCoreResult<ChangeSet> {
    if !window.descriptor.staged_mutation_handoff {
        return Err(EngineCoreError::InvalidDescriptor(
            "deferred writes require staged handoff window",
        ));
    }
    Ok(buffer.into_change_set(SmallVec::new()))
}

pub fn make_apply_payload(
    family_tag: FamilyTag,
    region_tag: RegionTag,
    batch_order: u64,
    change_set: ChangeSet,
) -> EngineCoreResult<ApplyPayload> {
    if batch_order == 0 {
        return Err(EngineCoreError::InvalidDescriptor(
            "batch order must be non-zero",
        ));
    }

    Ok(ApplyPayload {
        family_tag,
        region_tag,
        batch_order,
        flags: ApplyFlags::SEGMENTED,
        change_set,
    })
}
