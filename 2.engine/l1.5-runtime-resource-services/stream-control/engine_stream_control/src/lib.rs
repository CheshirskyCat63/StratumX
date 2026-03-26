use engine_core::{EngineCoreError, EngineCoreResult};
use std::collections::VecDeque;

pub const PAGE_TARGET_BYTES: usize = 128 * 1024;
pub const PAGE_HARD_CAP_BYTES: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamClass {
    Immediate,
    Predicted,
    Maintenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamRequest {
    pub id: u64,
    pub class: StreamClass,
    pub bytes: usize,
    pub locality_score: u16,
    pub chunk_span: u8,
}

#[derive(Debug, Default)]
pub struct StreamController {
    immediate: VecDeque<StreamRequest>,
    predicted: VecDeque<StreamRequest>,
    maintenance: VecDeque<StreamRequest>,
}

impl StreamController {
    pub fn activate(&mut self, req: StreamRequest) -> EngineCoreResult<()> {
        if req.bytes > PAGE_HARD_CAP_BYTES {
            return Err(EngineCoreError::InvalidDescriptor(
                "request exceeds canonical page hard cap",
            ));
        }
        if req.chunk_span > 2 {
            return Err(EngineCoreError::InvalidDescriptor(
                "prefetch may not span more than two adjacent chunk neighborhoods",
            ));
        }
        if !matches!(req.class, StreamClass::Immediate) && req.locality_score == 0 {
            return Err(EngineCoreError::InvariantViolation(
                "non-immediate request requires locality evidence",
            ));
        }

        let queue = match req.class {
            StreamClass::Immediate => &mut self.immediate,
            StreamClass::Predicted => &mut self.predicted,
            StreamClass::Maintenance => &mut self.maintenance,
        };

        let ceiling = match req.class {
            StreamClass::Immediate => 256,
            StreamClass::Predicted => 512,
            StreamClass::Maintenance => 256,
        };
        if queue.len() >= ceiling {
            return Err(EngineCoreError::InvalidDescriptor(
                "stream queue depth ceiling exceeded",
            ));
        }
        queue.push_back(req);
        Ok(())
    }

    pub fn schedule_next(&mut self) -> Option<StreamRequest> {
        self.immediate
            .pop_front()
            .or_else(|| self.predicted.pop_front())
            .or_else(|| self.maintenance.pop_front())
    }

    pub fn deny_optional_prefetch(&mut self) {
        self.predicted.clear();
    }

    pub fn queue_depths(&self) -> (usize, usize, usize) {
        (
            self.immediate.len(),
            self.predicted.len(),
            self.maintenance.len(),
        )
    }
}
