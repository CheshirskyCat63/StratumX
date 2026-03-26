//! Canonical L-0.5 storage mutation substrate.

use bitflags::bitflags;
use engine_handle::EntityHandle;
use engine_identity::{ComponentKind, EntityId};
use engine_storage_access::LayoutClass;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// Region-scoped tag for staged mutation batches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegionTag(pub u32);

/// Family-scoped tag for segmented apply lanes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FamilyTag(pub String);

/// Canonical stage lifecycle for mutation packages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationStage {
    Staged,
    Packaged,
    Applied,
}

bitflags! {
    /// Mutation flags retained while data is still staged.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct MutationFlags: u8 {
        const STRUCTURAL = 0b0000_0001;
        const DATA = 0b0000_0010;
        const IDEMPOTENT = 0b0000_0100;
        const TOMBSTONE = 0b0000_1000;
    }
}

/// One deferred write operation queued against a non-owning handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredWrite {
    pub target: EntityHandle,
    pub component: ComponentKind,
    pub bytes: SmallVec<[u8; 32]>,
    pub access_class: LayoutClass,
    pub stage: MutationStage,
    pub flags: MutationFlags,
}

impl DeferredWrite {
    /// Deferred writes are legal only while non-applied in this crate.
    pub fn validate(&self) -> Result<(), MutationValidationError> {
        if self.bytes.is_empty() {
            return Err(MutationValidationError::WritePayloadMustNotBeEmpty);
        }

        if matches!(self.stage, MutationStage::Applied) {
            return Err(MutationValidationError::AppliedStateForbiddenInMutationCrate);
        }

        Ok(())
    }
}

/// Batched mutation buffer with coalesce-friendly ordering.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MutationBuffer {
    writes: SmallVec<[DeferredWrite; 16]>,
}

impl MutationBuffer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stage(&mut self, write: DeferredWrite) -> Result<(), MutationValidationError> {
        write.validate()?;
        self.writes.push(write);
        Ok(())
    }

    /// Coalesces idempotent writes by target+component and retains insertion order class.
    pub fn coalesce(&mut self) {
        let mut compacted: SmallVec<[DeferredWrite; 16]> = SmallVec::new();

        for write in self.writes.drain(..) {
            if write.flags.contains(MutationFlags::IDEMPOTENT) {
                let existing = compacted.iter_mut().find(|candidate| {
                    candidate.target == write.target
                        && candidate.component == write.component
                        && candidate.flags.contains(MutationFlags::IDEMPOTENT)
                });
                if let Some(existing) = existing {
                    *existing = write;
                    continue;
                }
            }

            compacted.push(write);
        }

        self.writes = compacted;
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.writes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.writes.is_empty()
    }

    #[must_use]
    pub fn writes(&self) -> &[DeferredWrite] {
        &self.writes
    }

    pub fn into_change_set(
        self,
        tick: u64,
        region: RegionTag,
        family: FamilyTag,
    ) -> Result<ChangeSet, MutationValidationError> {
        ChangeSet::new(tick, region, family, self.writes)
    }
}

/// Structured package of staged storage changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSet {
    pub tick: u64,
    pub region: RegionTag,
    pub family: FamilyTag,
    pub writes: SmallVec<[DeferredWrite; 16]>,
}

impl ChangeSet {
    pub fn new(
        tick: u64,
        region: RegionTag,
        family: FamilyTag,
        writes: SmallVec<[DeferredWrite; 16]>,
    ) -> Result<Self, MutationValidationError> {
        if writes.is_empty() {
            return Err(MutationValidationError::ChangeSetRequiresWrites);
        }

        if writes
            .iter()
            .any(|write| matches!(write.stage, MutationStage::Applied))
        {
            return Err(MutationValidationError::AppliedStateForbiddenInMutationCrate);
        }

        Ok(Self {
            tick,
            region,
            family,
            writes,
        })
    }

    #[must_use]
    pub fn into_apply_payload(self) -> ApplyPayload {
        ApplyPayload {
            tick: self.tick,
            region: self.region,
            family: self.family,
            writes: self.writes,
        }
    }
}

/// Ordered apply feed prepared for a higher crate's authoritative integration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyPayload {
    pub tick: u64,
    pub region: RegionTag,
    pub family: FamilyTag,
    pub writes: SmallVec<[DeferredWrite; 16]>,
}

impl ApplyPayload {
    pub fn validate(&self) -> Result<(), MutationValidationError> {
        if self.writes.is_empty() {
            return Err(MutationValidationError::ApplyPayloadRequiresWrites);
        }

        if self
            .writes
            .iter()
            .any(|write| matches!(write.stage, MutationStage::Applied))
        {
            return Err(MutationValidationError::AppliedStateForbiddenInMutationCrate);
        }

        Ok(())
    }
}

/// Validation failures for staged mutation elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationValidationError {
    WritePayloadMustNotBeEmpty,
    AppliedStateForbiddenInMutationCrate,
    ChangeSetRequiresWrites,
    ApplyPayloadRequiresWrites,
}

/// Canonical helper proving this crate is non-owning over world truth.
#[must_use]
pub const fn is_world_non_owning(_: EntityId) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_handle::{HandleGeneration, HandleSlot, StableHandle};

    fn handle(entity: u64, slot: u32, generation: u32) -> EntityHandle {
        EntityHandle::new(
            EntityId(entity),
            StableHandle::new(HandleSlot(slot), HandleGeneration(generation)),
        )
    }

    fn write(stage: MutationStage, flags: MutationFlags, payload: &[u8]) -> DeferredWrite {
        DeferredWrite {
            target: handle(7, 2, 1),
            component: ComponentKind {
                name: "Transform".to_owned(),
                version: 1,
            },
            bytes: SmallVec::from_slice(payload),
            access_class: LayoutClass::DenseColumn,
            stage,
            flags,
        }
    }

    #[test]
    fn staged_mutation_buffer_accepts_non_applied_writes() {
        let mut buffer = MutationBuffer::new();
        let staged = write(MutationStage::Staged, MutationFlags::DATA, &[1, 2, 3]);

        assert_eq!(buffer.stage(staged), Ok(()));
        assert_eq!(buffer.len(), 1);
    }

    #[test]
    fn staged_mutation_buffer_rejects_applied_writes() {
        let mut buffer = MutationBuffer::new();
        let applied = write(MutationStage::Applied, MutationFlags::DATA, &[1, 2, 3]);

        assert_eq!(
            buffer.stage(applied),
            Err(MutationValidationError::AppliedStateForbiddenInMutationCrate)
        );
    }

    #[test]
    fn coalesce_replaces_idempotent_duplicates() {
        let mut buffer = MutationBuffer::new();
        let first = write(
            MutationStage::Staged,
            MutationFlags::DATA | MutationFlags::IDEMPOTENT,
            &[1],
        );
        let second = write(
            MutationStage::Packaged,
            MutationFlags::DATA | MutationFlags::IDEMPOTENT,
            &[9],
        );

        buffer.stage(first).unwrap();
        buffer.stage(second).unwrap();
        buffer.coalesce();

        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.writes()[0].bytes.as_slice(), &[9]);
    }
}
