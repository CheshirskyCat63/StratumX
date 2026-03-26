//! Canonical L-0.6 storage access substrate.

use bitflags::bitflags;
use engine_core::Tick;
use engine_handle::StableHandle;
use engine_storage_layout::{LayoutClass, LocalityClass};
use smallvec::SmallVec;

bitflags! {
    /// Compact legality flags for access descriptors.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AccessFlags: u8 {
        const READ = 0b0000_0001;
        const WRITE = 0b0000_0010;
        const STAGED = 0b0000_0100;
    }
}

/// Canonical access modes for deterministic legality checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    Read,
    Write,
    Staged,
    Mixed,
}

/// Access descriptor used by read views, write windows, and traversal entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessDescriptor {
    pub mode: AccessMode,
    pub flags: AccessFlags,
    pub requires_compiled_plan: bool,
}

impl AccessDescriptor {
    pub const fn validate(&self) -> Result<(), AccessValidationError> {
        match self.mode {
            AccessMode::Read => {
                if !self.flags.contains(AccessFlags::READ)
                    || self.flags.contains(AccessFlags::WRITE)
                {
                    return Err(AccessValidationError::ReadModeRequiresReadOnlyFlags);
                }
            }
            AccessMode::Write => {
                if !self.flags.contains(AccessFlags::WRITE)
                    || self.flags.contains(AccessFlags::READ)
                {
                    return Err(AccessValidationError::WriteModeRequiresWriteOnlyFlags);
                }
            }
            AccessMode::Staged => {
                if !self.flags.contains(AccessFlags::WRITE)
                    || !self.flags.contains(AccessFlags::STAGED)
                {
                    return Err(AccessValidationError::StagedModeRequiresStagedWriteFlags);
                }
            }
            AccessMode::Mixed => {
                if !(self.flags.contains(AccessFlags::READ)
                    && self.flags.contains(AccessFlags::WRITE))
                {
                    return Err(AccessValidationError::MixedModeRequiresReadAndWriteFlags);
                }
            }
        }

        Ok(())
    }

    #[must_use]
    pub const fn allows_write(&self) -> bool {
        self.flags.contains(AccessFlags::WRITE)
    }
}

/// Read-only deterministic access surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadView {
    pub handle: StableHandle,
    pub layout_class: LayoutClass,
    pub partition: u16,
    pub access: AccessDescriptor,
}

impl ReadView {
    pub fn validate(&self) -> Result<(), AccessValidationError> {
        self.access.validate()?;

        if self.partition == 0 {
            return Err(AccessValidationError::PartitionMustBeNonZero);
        }

        if !matches!(self.access.mode, AccessMode::Read) {
            return Err(AccessValidationError::ReadViewRequiresReadMode);
        }

        Ok(())
    }
}

/// Time-bounded mutable access window bound to legal traversal entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteWindowDescriptor {
    pub open_tick: Tick,
    pub close_tick: Tick,
    pub access: AccessDescriptor,
    pub traversal: TraversalEntryDescriptor,
}

impl WriteWindowDescriptor {
    pub fn validate(&self) -> Result<(), AccessValidationError> {
        self.access.validate()?;
        self.traversal.validate()?;

        if self.close_tick.0 < self.open_tick.0 {
            return Err(AccessValidationError::WriteWindowTickOrderInvalid);
        }

        if !self.access.allows_write() {
            return Err(AccessValidationError::WriteWindowRequiresWriteAccess);
        }

        Ok(())
    }
}

/// Deterministic traversal access entry descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraversalEntryDescriptor {
    pub plan_id: u64,
    pub locality: LocalityClass,
    pub partitions: SmallVec<[u16; 4]>,
    pub access: AccessDescriptor,
    pub compiled_plan_only: bool,
    pub staged_handoff_only: bool,
}

impl TraversalEntryDescriptor {
    pub fn validate(&self) -> Result<(), AccessValidationError> {
        self.access.validate()?;

        if self.plan_id == 0 {
            return Err(AccessValidationError::TraversalPlanIdMustBeNonZero);
        }

        if self.partitions.is_empty() || self.partitions.contains(&0) {
            return Err(AccessValidationError::TraversalPartitionsMustBeNonZero);
        }

        if !self.compiled_plan_only {
            return Err(AccessValidationError::TraversalEntryRequiresCompiledPlan);
        }

        if self.access.allows_write() && !self.staged_handoff_only {
            return Err(AccessValidationError::TraversalWriteRequiresStagedHandoff);
        }

        Ok(())
    }
}

/// Access legality validation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessValidationError {
    ReadModeRequiresReadOnlyFlags,
    WriteModeRequiresWriteOnlyFlags,
    StagedModeRequiresStagedWriteFlags,
    MixedModeRequiresReadAndWriteFlags,
    PartitionMustBeNonZero,
    ReadViewRequiresReadMode,
    WriteWindowTickOrderInvalid,
    WriteWindowRequiresWriteAccess,
    TraversalPlanIdMustBeNonZero,
    TraversalPartitionsMustBeNonZero,
    TraversalEntryRequiresCompiledPlan,
    TraversalWriteRequiresStagedHandoff,
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_handle::{HandleGeneration, HandleSlot, StableHandle};

    fn handle() -> StableHandle {
        StableHandle::new(HandleSlot(1), HandleGeneration(1))
    }

    fn read_access() -> AccessDescriptor {
        AccessDescriptor {
            mode: AccessMode::Read,
            flags: AccessFlags::READ,
            requires_compiled_plan: true,
        }
    }

    fn staged_write_access() -> AccessDescriptor {
        AccessDescriptor {
            mode: AccessMode::Staged,
            flags: AccessFlags::WRITE | AccessFlags::STAGED,
            requires_compiled_plan: true,
        }
    }

    #[test]
    fn read_view_validation_requires_read_mode_and_partition() {
        let view = ReadView {
            handle: handle(),
            layout_class: LayoutClass::Sparse,
            partition: 1,
            access: read_access(),
        };

        assert_eq!(view.validate(), Ok(()));
    }

    #[test]
    fn traversal_entry_requires_compiled_plan_for_legality() {
        let traversal = TraversalEntryDescriptor {
            plan_id: 42,
            locality: LocalityClass::Partition,
            partitions: SmallVec::from_slice(&[1, 2]),
            access: staged_write_access(),
            compiled_plan_only: false,
            staged_handoff_only: true,
        };

        assert_eq!(
            traversal.validate(),
            Err(AccessValidationError::TraversalEntryRequiresCompiledPlan)
        );
    }

    #[test]
    fn write_window_requires_non_decreasing_tick_range_and_write_access() {
        let traversal = TraversalEntryDescriptor {
            plan_id: 7,
            locality: LocalityClass::TraversalLane,
            partitions: SmallVec::from_slice(&[1]),
            access: staged_write_access(),
            compiled_plan_only: true,
            staged_handoff_only: true,
        };

        let window = WriteWindowDescriptor {
            open_tick: Tick(10),
            close_tick: Tick(11),
            access: staged_write_access(),
            traversal,
        };

        assert_eq!(window.validate(), Ok(()));
    }
}
