use engine_core::Tick;
use engine_handle::{HandleGeneration, HandleSlot, StableHandle};
use engine_storage_access::{
    AccessDescriptor, AccessFlags, AccessMode, AccessValidationError, ReadView,
    TraversalEntryDescriptor, WriteWindowDescriptor,
};
use engine_storage_layout::{LayoutClass, LocalityClass};
use smallvec::smallvec;

fn staged_descriptor() -> AccessDescriptor {
    AccessDescriptor {
        mode: AccessMode::Staged,
        flags: AccessFlags::WRITE | AccessFlags::STAGED,
        requires_compiled_plan: true,
    }
}

#[test]
fn access_mode_legality_rejects_mixed_mode_without_write_flag() {
    let descriptor = AccessDescriptor {
        mode: AccessMode::Mixed,
        flags: AccessFlags::READ,
        requires_compiled_plan: true,
    };

    assert_eq!(
        descriptor.validate(),
        Err(AccessValidationError::MixedModeRequiresReadAndWriteFlags)
    );
}

#[test]
fn read_view_rejects_non_read_mode() {
    let view = ReadView {
        handle: StableHandle::new(HandleSlot(9), HandleGeneration(1)),
        layout_class: LayoutClass::DenseColumn,
        partition: 1,
        access: staged_descriptor(),
    };

    assert_eq!(
        view.validate(),
        Err(AccessValidationError::ReadViewRequiresReadMode)
    );
}

#[test]
fn traversal_write_requires_staged_handoff() {
    let traversal = TraversalEntryDescriptor {
        plan_id: 9,
        locality: LocalityClass::Cache,
        partitions: smallvec![1],
        access: staged_descriptor(),
        compiled_plan_only: true,
        staged_handoff_only: false,
    };

    assert_eq!(
        traversal.validate(),
        Err(AccessValidationError::TraversalWriteRequiresStagedHandoff)
    );
}

#[test]
fn write_window_rejects_reverse_tick_order() {
    let traversal = TraversalEntryDescriptor {
        plan_id: 9,
        locality: LocalityClass::Cache,
        partitions: smallvec![1],
        access: staged_descriptor(),
        compiled_plan_only: true,
        staged_handoff_only: true,
    };

    let window = WriteWindowDescriptor {
        open_tick: Tick(5),
        close_tick: Tick(4),
        access: staged_descriptor(),
        traversal,
    };

    assert_eq!(
        window.validate(),
        Err(AccessValidationError::WriteWindowTickOrderInvalid)
    );
}
