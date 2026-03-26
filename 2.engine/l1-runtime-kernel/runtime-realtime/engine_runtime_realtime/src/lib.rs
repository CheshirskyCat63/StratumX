use engine_core::EngineCoreResult;
use engine_runtime::{RuntimeKernel, RuntimeProfile};
use engine_world::WorldState;

pub const REALTIME_FRAME_HZ: u32 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationPolicy {
    LowLatency,
    ThroughputPreferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealtimeRole {
    InteractiveClient,
    ListenHost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RealtimeOutputs {
    pub frame: u64,
    pub cadence_hz: u32,
    pub presentable: bool,
}

#[derive(Debug)]
pub struct RealtimeRuntime {
    role: RealtimeRole,
    policy: PresentationPolicy,
    kernel: RuntimeKernel,
}

impl RealtimeRuntime {
    pub fn new(role: RealtimeRole, policy: PresentationPolicy) -> Self {
        let profile = match role {
            RealtimeRole::InteractiveClient => RuntimeProfile::Interactive60,
            RealtimeRole::ListenHost => RuntimeProfile::ListenHost60,
        };
        Self {
            role,
            policy,
            kernel: RuntimeKernel::new(profile),
        }
    }

    pub fn role(&self) -> RealtimeRole {
        self.role
    }
    pub fn policy(&self) -> PresentationPolicy {
        self.policy
    }

    pub fn start(&mut self) {
        self.kernel.start();
    }

    pub fn step_frame(&mut self, world: &mut WorldState) -> EngineCoreResult<RealtimeOutputs> {
        self.kernel.queue_presentable_frame()?;
        self.kernel.advance_tick(world)?;
        Ok(RealtimeOutputs {
            frame: self.kernel.tick().0,
            cadence_hz: REALTIME_FRAME_HZ,
            presentable: true,
        })
    }
}
