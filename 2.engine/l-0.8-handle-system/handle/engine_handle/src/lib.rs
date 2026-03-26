//! Canonical L-0.8 handle substrate.
//!
//! This crate defines non-owning, stable temporal references and generation-aware
//! validation/invalidation helpers.

use engine_identity::EntityId;
use serde::{Deserialize, Serialize};

/// Stable slot coordinate for a handle family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HandleSlot(pub u32);

/// Monotonic generation used to detect stale temporal references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct HandleGeneration(pub u32);

/// Stable temporal token for an identity-bearing value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StableHandle {
    pub slot: HandleSlot,
    pub generation: HandleGeneration,
}

impl StableHandle {
    #[must_use]
    pub const fn new(slot: HandleSlot, generation: HandleGeneration) -> Self {
        Self { slot, generation }
    }
}

/// Public non-owning handle to an entity identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityHandle {
    pub entity: EntityId,
    pub stable: StableHandle,
}

impl EntityHandle {
    #[must_use]
    pub const fn new(entity: EntityId, stable: StableHandle) -> Self {
        Self { entity, stable }
    }

    /// Handles never imply ownership of the target identity/state.
    #[must_use]
    pub const fn is_non_owning(&self) -> bool {
        true
    }
}

/// Invalidation marker describing whether a handle lineage remains authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidationMarker {
    current_generation: HandleGeneration,
    invalidated: bool,
}

impl InvalidationMarker {
    #[must_use]
    pub const fn new(current_generation: HandleGeneration) -> Self {
        Self {
            current_generation,
            invalidated: false,
        }
    }

    #[must_use]
    pub const fn current_generation(&self) -> HandleGeneration {
        self.current_generation
    }

    #[must_use]
    pub const fn is_invalidated(&self) -> bool {
        self.invalidated
    }

    pub fn bump_generation(&mut self) {
        self.current_generation.0 = self.current_generation.0.saturating_add(1);
    }

    pub fn invalidate(&mut self) {
        self.invalidated = true;
    }
}

/// Canonical outcome of generation-aware handle validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleValidation {
    Valid,
    Invalidated,
    Stale {
        expected: HandleGeneration,
        observed: HandleGeneration,
    },
}

#[must_use]
pub fn validate_handle(handle: StableHandle, marker: InvalidationMarker) -> HandleValidation {
    if marker.invalidated {
        return HandleValidation::Invalidated;
    }

    if is_stale_generation(handle.generation, marker.current_generation) {
        return HandleValidation::Stale {
            expected: marker.current_generation,
            observed: handle.generation,
        };
    }

    HandleValidation::Valid
}

/// Returns true when a handle generation is stale against the authoritative generation.
#[must_use]
pub const fn is_stale_generation(
    handle_generation: HandleGeneration,
    current_generation: HandleGeneration,
) -> bool {
    handle_generation.0 != current_generation.0
}

/// Returns true if a stable handle is stale under the supplied marker.
#[must_use]
pub fn is_stale_handle(handle: StableHandle, marker: InvalidationMarker) -> bool {
    matches!(
        validate_handle(handle, marker),
        HandleValidation::Stale { .. } | HandleValidation::Invalidated
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_matching_generation_when_active() {
        let handle = StableHandle::new(HandleSlot(7), HandleGeneration(3));
        let marker = InvalidationMarker::new(HandleGeneration(3));

        assert_eq!(validate_handle(handle, marker), HandleValidation::Valid);
    }

    #[test]
    fn marks_stale_after_generation_turnover() {
        let handle = StableHandle::new(HandleSlot(7), HandleGeneration(3));
        let mut marker = InvalidationMarker::new(HandleGeneration(3));
        marker.bump_generation();

        assert_eq!(
            validate_handle(handle, marker),
            HandleValidation::Stale {
                expected: HandleGeneration(4),
                observed: HandleGeneration(3)
            }
        );
        assert!(is_stale_handle(handle, marker));
    }

    #[test]
    fn marks_invalidated_when_marker_explicitly_invalidated() {
        let handle = StableHandle::new(HandleSlot(7), HandleGeneration(3));
        let mut marker = InvalidationMarker::new(HandleGeneration(3));
        marker.invalidate();

        assert_eq!(
            validate_handle(handle, marker),
            HandleValidation::Invalidated
        );
        assert!(is_stale_handle(handle, marker));
    }

    #[test]
    fn entity_handle_is_explicitly_non_owning() {
        let handle = EntityHandle::new(
            EntityId(17),
            StableHandle::new(HandleSlot(1), HandleGeneration(0)),
        );

        assert!(handle.is_non_owning());
    }
}
