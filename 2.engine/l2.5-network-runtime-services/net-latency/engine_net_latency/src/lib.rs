use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputSample {
    pub tick: u32,
    pub payload: u16,
}

#[derive(Default)]
pub struct LatencyService {
    history: SmallVec<[InputSample; 32]>,
}

impl LatencyService {
    pub fn push_input(&mut self, sample: InputSample) -> EngineCoreResult<()> {
        if self.history.len() >= 256 {
            self.history.remove(0);
        }
        self.history.push(sample);
        Ok(())
    }
    pub fn predict_tick(&self, current_tick: u32, rtt_ms: u16) -> u32 {
        current_tick + u32::from(rtt_ms / 50)
    }
    pub fn validate_rewind_window(&self, window_ms: u16) -> EngineCoreResult<()> {
        if window_ms > 500 {
            return Err(EngineCoreError::InvalidDescriptor(
                "rewind window exceeds canonical validation window",
            ));
        }
        Ok(())
    }
}
