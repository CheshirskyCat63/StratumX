use engine_core::{EngineCoreError, EngineCoreResult};
use smallvec::SmallVec;
use std::collections::BTreeMap;

pub const UPLOAD_BYTES_P95: usize = 32 * 1024 * 1024;
pub const UPLOAD_BYTES_P99: usize = 64 * 1024 * 1024;
pub const COMPLETION_QUEUE_CEILING: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeClass {
    TextureTranscodeReady,
    GeometryBinary,
    StreamingAudioCompressed,
    RawAuthoredAsset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferDescriptor {
    pub id: u64,
    pub decode_class: DecodeClass,
    pub bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferBatch {
    pub transfers: SmallVec<[TransferDescriptor; 8]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferFence {
    pub id: u64,
    pub visible: bool,
}

#[derive(Debug, Default)]
pub struct TransferController {
    quarantined: BTreeMap<u64, bool>,
    completion_queue: SmallVec<[u64; 64]>,
}

impl TransferController {
    pub fn submit_batch(&mut self, batch: &TransferBatch) -> EngineCoreResult<()> {
        let frame_bytes: usize = batch.transfers.iter().map(|t| t.bytes).sum();
        if frame_bytes > UPLOAD_BYTES_P99 {
            return Err(EngineCoreError::InvalidDescriptor(
                "upload bytes exceeded p99 ceiling",
            ));
        }

        for transfer in &batch.transfers {
            if transfer.decode_class == DecodeClass::RawAuthoredAsset {
                return Err(EngineCoreError::InvariantViolation(
                    "raw authored asset decode is runtime-illegal",
                ));
            }
            self.quarantined.insert(transfer.id, true);
        }

        Ok(())
    }

    pub fn mark_fence_visible(
        &mut self,
        fence: TransferFence,
        transfer_ids: &[u64],
    ) -> EngineCoreResult<()> {
        if !fence.visible {
            return Err(EngineCoreError::InvariantViolation(
                "fence must be visible before release",
            ));
        }
        if self.completion_queue.len() + transfer_ids.len() > COMPLETION_QUEUE_CEILING {
            return Err(EngineCoreError::InvalidDescriptor(
                "transfer completion queue depth exceeded",
            ));
        }
        for id in transfer_ids {
            if let Some(state) = self.quarantined.get_mut(id) {
                *state = false;
            }
            self.completion_queue.push(*id);
        }
        Ok(())
    }

    pub fn release_ready(&mut self) -> SmallVec<[u64; 64]> {
        let mut released = SmallVec::new();
        let mut kept = SmallVec::new();
        for id in self.completion_queue.drain(..) {
            if self.quarantined.get(&id).copied() == Some(false) {
                released.push(id);
                self.quarantined.remove(&id);
            } else {
                kept.push(id);
            }
        }
        self.completion_queue = kept;
        released
    }
}
