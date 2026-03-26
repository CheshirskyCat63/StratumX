use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDelta {
    pub region: u32,
    pub fluid: f32,
    pub thermal: f32,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuralProduct {
    pub region: u32,
    pub stress: f32,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtmosphericProduct {
    pub region: u32,
    pub wind_m_s: f32,
}

#[derive(Default)]
pub struct FieldFamily {
    deltas: SmallVec<[FieldDelta; 16]>,
}

impl FieldFamily {
    pub fn push_delta(&mut self, delta: FieldDelta) -> EngineCoreResult<()> {
        if self.deltas.len() >= 256 {
            return Err(EngineCoreError::InvariantViolation(
                "field delta ceiling exceeded",
            ));
        }
        self.deltas.push(delta);
        Ok(())
    }
    pub fn staged_deltas(&self) -> &[FieldDelta] {
        &self.deltas
    }
    pub fn structural(&self, region: u32) -> StructuralProduct {
        StructuralProduct {
            region,
            stress: 0.5,
        }
    }
    pub fn atmosphere(&self, region: u32) -> AtmosphericProduct {
        AtmosphericProduct {
            region,
            wind_m_s: 2.0,
        }
    }
}
