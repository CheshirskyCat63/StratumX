use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeProfile {
    Interactive60,
    ListenHost60,
    Headless20,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRole {
    Local,
    InteractiveHostAware,
    ListenHost,
    HeadlessHost,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupConfig {
    pub profile: RuntimeProfile,
    pub role: NetworkRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportEpoch {
    pub id: u64,
    pub invalidated: bool,
}

pub fn bootstrap(config: &StartupConfig) -> EngineCoreResult<ExportEpoch> {
    match (config.profile, config.role) {
        (RuntimeProfile::Headless20, NetworkRole::InteractiveHostAware) => {
            Err(EngineCoreError::InvalidDescriptor(
                "headless profile cannot use interactive host-aware role",
            ))
        }
        _ => Ok(ExportEpoch {
            id: 1,
            invalidated: false,
        }),
    }
}

pub fn invalidate(epoch: &mut ExportEpoch) {
    epoch.invalidated = true;
}
