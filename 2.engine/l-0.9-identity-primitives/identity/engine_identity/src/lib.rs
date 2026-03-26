//! Canonical L-0.9 identity primitives for StratumX.

use serde::{Deserialize, Serialize};
use slotmap::{new_key_type, Key};

pub use engine_core as core;

new_key_type! {
    /// Internal stable key substrate for entities.
    pub struct EntityKey;

    /// Internal stable key substrate for components.
    pub struct ComponentKey;
}

/// Typed identity namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IdentityDomain {
    /// Entity identity namespace.
    Entity,
    /// Component identity namespace.
    Component,
}

/// Lifecycle generation value carried by identity tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Generation(pub u32);

impl Generation {
    /// First legal generation for newly issued identities.
    pub const INITIAL: Self = Self(1);

    /// Returns the next generation, wrapping while preserving a non-zero value.
    #[must_use]
    pub fn next(self) -> Self {
        let next = self.0.wrapping_add(1);
        if next == 0 {
            Self::INITIAL
        } else {
            Self(next)
        }
    }
}

/// Canonical entity identity token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId {
    key: EntityKey,
    generation: Generation,
}

impl EntityId {
    /// Domain tag for entity identifiers.
    pub const DOMAIN: IdentityDomain = IdentityDomain::Entity;

    /// Builds an entity identifier from raw key/generation values.
    #[must_use]
    pub fn new(key: EntityKey, generation: Generation) -> Self {
        Self { key, generation }
    }

    /// Returns the stable key portion of the identifier.
    #[must_use]
    pub fn key(self) -> EntityKey {
        self.key
    }

    /// Returns the generation portion of the identifier.
    #[must_use]
    pub fn generation(self) -> Generation {
        self.generation
    }

    /// Validates whether this identifier matches the current generation.
    pub fn validate_generation(self, current: Generation) -> Result<(), IdentityError> {
        validate_generation(self.generation, current)
    }

    /// Validates whether a caller-provided domain matches this identifier.
    pub fn validate_domain(self, domain: IdentityDomain) -> Result<(), IdentityError> {
        validate_domain(Self::DOMAIN, domain)
    }
}

/// Canonical component identity token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId {
    key: ComponentKey,
    generation: Generation,
}

impl ComponentId {
    /// Domain tag for component identifiers.
    pub const DOMAIN: IdentityDomain = IdentityDomain::Component;

    /// Builds a component identifier from raw key/generation values.
    #[must_use]
    pub fn new(key: ComponentKey, generation: Generation) -> Self {
        Self { key, generation }
    }

    /// Returns the stable key portion of the identifier.
    #[must_use]
    pub fn key(self) -> ComponentKey {
        self.key
    }

    /// Returns the generation portion of the identifier.
    #[must_use]
    pub fn generation(self) -> Generation {
        self.generation
    }

    /// Validates whether this identifier matches the current generation.
    pub fn validate_generation(self, current: Generation) -> Result<(), IdentityError> {
        validate_generation(self.generation, current)
    }

    /// Validates whether a caller-provided domain matches this identifier.
    pub fn validate_domain(self, domain: IdentityDomain) -> Result<(), IdentityError> {
        validate_domain(Self::DOMAIN, domain)
    }
}

/// Reuse-delay primitive for generation-aware identity lifecycle control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReuseDelay {
    delay_epochs: u64,
}

impl ReuseDelay {
    /// Creates a new delay policy measured in epochs.
    #[must_use]
    pub fn new(delay_epochs: u64) -> Self {
        Self { delay_epochs }
    }

    /// Returns true when a retired identity can be legally reused.
    #[must_use]
    pub fn can_reuse(self, current_epoch: u64, retired_epoch: u64) -> bool {
        current_epoch.saturating_sub(retired_epoch) >= self.delay_epochs
    }
}

/// Validation failures for stale/invalid identity use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityError {
    /// The provided generation no longer matches the current generation.
    StaleIdentity {
        /// Generation embedded in the incoming identifier.
        provided: Generation,
        /// Current generation tracked by the owning system.
        current: Generation,
    },
    /// The caller attempted to use an identity in the wrong namespace.
    InvalidDomain {
        /// Domain that was required.
        expected: IdentityDomain,
        /// Domain that was supplied by caller/context.
        found: IdentityDomain,
    },
}

/// Validates generation freshness against the current generation.
pub fn validate_generation(provided: Generation, current: Generation) -> Result<(), IdentityError> {
    if provided == current {
        Ok(())
    } else {
        Err(IdentityError::StaleIdentity { provided, current })
    }
}

/// Validates that a supplied identity domain matches the expected namespace.
pub fn validate_domain(
    expected: IdentityDomain,
    found: IdentityDomain,
) -> Result<(), IdentityError> {
    if expected == found {
        Ok(())
    } else {
        Err(IdentityError::InvalidDomain { expected, found })
    }
}

/// Converts a slotmap key into a stable raw key payload.
#[must_use]
pub fn key_to_raw<K: Key>(key: K) -> u64 {
    key.data().as_ffi()
}
