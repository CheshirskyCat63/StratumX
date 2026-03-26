use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterestEntry {
    pub connection_id: u64,
    pub region: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncSnapshot {
    pub connection_id: u64,
    pub bytes: u16,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncDelta {
    pub connection_id: u64,
    pub bytes: u16,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AckState {
    pub connection_id: u64,
    pub ack_window: u16,
}

#[derive(Default)]
pub struct NetSyncService {
    interest: SmallVec<[InterestEntry; 16]>,
}

impl NetSyncService {
    pub fn register_interest(&mut self, entry: InterestEntry) -> EngineCoreResult<()> {
        if self.interest.len() >= 2048 {
            return Err(EngineCoreError::InvariantViolation(
                "interest ceiling exceeded",
            ));
        }
        self.interest.push(entry);
        Ok(())
    }
    pub fn build_snapshot(&self, connection_id: u64, bytes: u16) -> EngineCoreResult<SyncSnapshot> {
        if bytes > 1400 {
            return Err(EngineCoreError::InvalidDescriptor("snapshot over budget"));
        }
        Ok(SyncSnapshot {
            connection_id,
            bytes,
        })
    }
    pub fn build_delta(&self, connection_id: u64, bytes: u16) -> EngineCoreResult<SyncDelta> {
        if bytes > 1000 {
            return Err(EngineCoreError::InvalidDescriptor("delta over budget"));
        }
        Ok(SyncDelta {
            connection_id,
            bytes,
        })
    }
    pub fn ack(&self, connection_id: u64, ack_window: u16) -> EngineCoreResult<AckState> {
        if ack_window > 256 {
            return Err(EngineCoreError::InvalidDescriptor(
                "ack window exceeds canonical ceiling",
            ));
        }
        Ok(AckState {
            connection_id,
            ack_window,
        })
    }
}
