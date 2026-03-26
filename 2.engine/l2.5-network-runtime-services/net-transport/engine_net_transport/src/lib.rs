use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketLane {
    Control,
    Bulk,
    State,
    Input,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketDescriptor {
    pub lane: PacketLane,
    pub bytes: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportSession {
    pub id: u64,
    pub authenticated: bool,
}

#[derive(Default)]
pub struct TransportService {
    sessions: SmallVec<[TransportSession; 8]>,
}

impl TransportService {
    pub fn open_session(&mut self, session: TransportSession) -> EngineCoreResult<()> {
        if self.sessions.len() >= 64 {
            return Err(EngineCoreError::InvariantViolation(
                "session ceiling exceeded",
            ));
        }
        if !session.authenticated {
            return Err(EngineCoreError::InvalidDescriptor(
                "session must be authenticated",
            ));
        }
        self.sessions.push(session);
        Ok(())
    }
    pub fn publish(&self, packet: &PacketDescriptor) -> EngineCoreResult<()> {
        match packet.lane {
            PacketLane::Control if packet.bytes > 1200 => Err(EngineCoreError::InvalidDescriptor(
                "control packet too large",
            )),
            PacketLane::State | PacketLane::Input if packet.bytes > 1000 => Err(
                EngineCoreError::InvalidDescriptor("unreliable packet too large"),
            ),
            _ => Ok(()),
        }
    }
}
