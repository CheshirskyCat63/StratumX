use crate::error::{EngineCoreError, EngineCoreResult};

/// Minimal descriptor contract consumed by upper crates.
pub trait EngineDescriptor {
    fn validate(&self) -> EngineCoreResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticDescriptor {
    pub label: String,
}

impl EngineDescriptor for StaticDescriptor {
    fn validate(&self) -> EngineCoreResult<()> {
        if self.label.trim().is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "label must not be empty",
            ));
        }

        Ok(())
    }
}
