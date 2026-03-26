use engine_runtime::{RuntimeKernel, RuntimeProfile};
use engine_world::WorldState;

pub const HEADLESS_CADENCE_HZ: u32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadlessRole {
    DedicatedServer,
    BatchSimulation,
    OfflineVerifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeadlessOutputs {
    pub tick: u64,
    pub cadence_hz: u32,
}

#[derive(Debug)]
pub struct HeadlessRuntime {
    role: HeadlessRole,
    kernel: RuntimeKernel,
}

impl HeadlessRuntime {
    pub fn new(role: HeadlessRole) -> Self {
        Self {
            role,
            kernel: RuntimeKernel::new(RuntimeProfile::Headless20),
        }
    }

    pub fn role(&self) -> HeadlessRole {
        self.role
    }

    pub fn start(&mut self) {
        self.kernel.start();
    }

    pub fn step(
        &mut self,
        world: &mut WorldState,
    ) -> engine_core::EngineCoreResult<HeadlessOutputs> {
        self.kernel.advance_tick(world)?;
        Ok(HeadlessOutputs {
            tick: self.kernel.tick().0,
            cadence_hz: HEADLESS_CADENCE_HZ,
        })
    }
}
