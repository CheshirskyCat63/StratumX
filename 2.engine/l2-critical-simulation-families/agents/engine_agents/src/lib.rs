use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationProduct {
    pub agent: u64,
    pub waypoint_region: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerceptionProduct {
    pub agent: u64,
    pub tier: u8,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionIntent {
    pub agent: u64,
    pub action: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SocietyDelta {
    pub cohort: u32,
    pub influence_delta: i16,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduleProduct {
    pub agent: u64,
    pub next_tick: u32,
}

#[derive(Default)]
pub struct AgentsFamily {
    intents: SmallVec<[ActionIntent; 16]>,
}

impl AgentsFamily {
    pub fn enqueue_intent(&mut self, intent: ActionIntent) -> EngineCoreResult<()> {
        if self.intents.len() >= 1024 {
            return Err(EngineCoreError::InvariantViolation(
                "intent ceiling exceeded",
            ));
        }
        self.intents.push(intent);
        Ok(())
    }
    pub fn intents(&self) -> &[ActionIntent] {
        &self.intents
    }
    pub fn navigation(&self, agent: u64, waypoint_region: u32) -> NavigationProduct {
        NavigationProduct {
            agent,
            waypoint_region,
        }
    }
    pub fn perception(&self, agent: u64, near: bool) -> PerceptionProduct {
        PerceptionProduct {
            agent,
            tier: if near { 0 } else { 1 },
        }
    }
    pub fn society(&self, cohort: u32, influence_delta: i16) -> SocietyDelta {
        SocietyDelta {
            cohort,
            influence_delta,
        }
    }
    pub fn schedule(&self, agent: u64, next_tick: u32) -> ScheduleProduct {
        ScheduleProduct { agent, next_tick }
    }
}
