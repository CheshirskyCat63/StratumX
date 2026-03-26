use engine_core::{EngineCoreError, EngineCoreResult, Tick};
use engine_world::{ApplySegment, WorldState};
use smallvec::SmallVec;
use std::collections::VecDeque;

pub const MAX_APPLY_RECORDS_PER_SEGMENT: usize = 16_384;
pub const MAX_APPLY_RECORDS_AGGREGATE: usize = 65_536;
pub const MAX_TRANSFER_COMPLETIONS: usize = 4_096;
pub const MAX_CONNECTION_RECORDS: usize = 1_024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeLifecycle {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimePhase {
    Ingress,
    Read,
    Compute,
    Resource,
    AuthoritySync,
    Stage,
    Apply,
    Egress,
    Diagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeProfile {
    Interactive60,
    ListenHost60,
    Headless20,
}

#[derive(Debug, Clone)]
pub struct ApplyRecord {
    pub source_tick: Tick,
    pub segment: ApplySegment,
}

#[derive(Debug)]
pub struct RuntimeKernel {
    lifecycle: RuntimeLifecycle,
    profile: RuntimeProfile,
    phase: RuntimePhase,
    tick: Tick,
    apply_queue: VecDeque<ApplyRecord>,
    transfer_completion_queue: VecDeque<u64>,
    connection_publication_queue: VecDeque<u64>,
    presentable_frame_queued: bool,
}

impl RuntimeKernel {
    pub fn new(profile: RuntimeProfile) -> Self {
        Self {
            lifecycle: RuntimeLifecycle::Stopped,
            profile,
            phase: RuntimePhase::Ingress,
            tick: Tick(0),
            apply_queue: VecDeque::new(),
            transfer_completion_queue: VecDeque::new(),
            connection_publication_queue: VecDeque::new(),
            presentable_frame_queued: false,
        }
    }

    pub fn lifecycle(&self) -> RuntimeLifecycle {
        self.lifecycle
    }
    pub fn profile(&self) -> RuntimeProfile {
        self.profile
    }
    pub fn phase(&self) -> RuntimePhase {
        self.phase
    }
    pub fn tick(&self) -> Tick {
        self.tick
    }

    pub fn start(&mut self) {
        self.lifecycle = RuntimeLifecycle::Running;
    }
    pub fn pause(&mut self) {
        self.lifecycle = RuntimeLifecycle::Paused;
    }
    pub fn resume(&mut self) {
        self.lifecycle = RuntimeLifecycle::Running;
    }
    pub fn stop(&mut self) {
        self.lifecycle = RuntimeLifecycle::Stopped;
    }

    pub fn canonical_phase_order() -> SmallVec<[RuntimePhase; 9]> {
        SmallVec::from_buf([
            RuntimePhase::Ingress,
            RuntimePhase::Read,
            RuntimePhase::Compute,
            RuntimePhase::Resource,
            RuntimePhase::AuthoritySync,
            RuntimePhase::Stage,
            RuntimePhase::Apply,
            RuntimePhase::Egress,
            RuntimePhase::Diagnostics,
        ])
    }

    pub fn enqueue_apply(&mut self, record: ApplyRecord) -> EngineCoreResult<()> {
        if self.apply_queue.len() >= MAX_APPLY_RECORDS_AGGREGATE {
            return Err(EngineCoreError::InvalidDescriptor(
                "apply queue aggregate ceiling exceeded",
            ));
        }
        if record.segment.family_tags.len() > MAX_APPLY_RECORDS_PER_SEGMENT {
            return Err(EngineCoreError::InvalidDescriptor(
                "apply segment exceeds per-segment ceiling",
            ));
        }
        self.apply_queue.push_back(record);
        Ok(())
    }

    pub fn enqueue_transfer_completion(&mut self, completion_id: u64) -> EngineCoreResult<()> {
        if self.transfer_completion_queue.len() >= MAX_TRANSFER_COMPLETIONS {
            return Err(EngineCoreError::InvalidDescriptor(
                "transfer completion queue depth exceeded",
            ));
        }
        self.transfer_completion_queue.push_back(completion_id);
        Ok(())
    }

    pub fn enqueue_connection_publication(&mut self, publication_id: u64) -> EngineCoreResult<()> {
        if self.connection_publication_queue.len() >= MAX_CONNECTION_RECORDS {
            return Err(EngineCoreError::InvalidDescriptor(
                "connection publication queue depth exceeded",
            ));
        }
        self.connection_publication_queue.push_back(publication_id);
        Ok(())
    }

    pub fn queue_presentable_frame(&mut self) -> EngineCoreResult<()> {
        if self.presentable_frame_queued {
            return Err(EngineCoreError::InvalidDescriptor(
                "presentable frame queue depth exceeded",
            ));
        }
        self.presentable_frame_queued = true;
        Ok(())
    }

    pub fn advance_tick(&mut self, world: &mut WorldState) -> EngineCoreResult<usize> {
        if self.lifecycle != RuntimeLifecycle::Running {
            return Err(EngineCoreError::InvariantViolation(
                "runtime must be running",
            ));
        }

        for phase in Self::canonical_phase_order() {
            self.phase = phase;
            if phase == RuntimePhase::Apply {
                self.apply(world)?;
            }
        }

        self.tick = Tick(self.tick.0.saturating_add(1));
        self.presentable_frame_queued = false;
        Ok(self.apply_queue.len())
    }

    fn apply(&mut self, world: &mut WorldState) -> EngineCoreResult<()> {
        let mut to_apply = Vec::new();
        while let Some(record) = self.apply_queue.pop_front() {
            let age = self.tick.0.saturating_sub(record.source_tick.0);
            if age > 1 {
                return Err(EngineCoreError::InvariantViolation(
                    "apply record exceeded tick-age ceiling",
                ));
            }
            to_apply.push(record.segment);
        }
        world.apply(&to_apply, 1)
    }
}
