use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectileLane {
    Near,
    Far,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactProduct {
    pub body_a: u64,
    pub body_b: u64,
    pub penetration_mm: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionDelta {
    pub body: u64,
    pub velocity_cm_s: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrajectoryProduct {
    pub projectile: u64,
    pub lane: ProjectileLane,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactProduct {
    pub projectile: u64,
    pub material: u16,
}

#[derive(Default)]
pub struct KineticsFamily {
    contacts: SmallVec<[ContactProduct; 16]>,
}

impl KineticsFamily {
    pub fn record_contact(&mut self, contact: ContactProduct) -> EngineCoreResult<()> {
        if self.contacts.len() >= 512 {
            return Err(EngineCoreError::InvariantViolation(
                "contact ceiling exceeded",
            ));
        }
        self.contacts.push(contact);
        Ok(())
    }

    pub fn solve_motion(&self) -> SmallVec<[MotionDelta; 16]> {
        self.contacts
            .iter()
            .map(|c| MotionDelta {
                body: c.body_a,
                velocity_cm_s: -(c.penetration_mm as i32),
            })
            .collect()
    }

    pub fn solve_trajectory(&self, projectile: u64, near: bool) -> TrajectoryProduct {
        TrajectoryProduct {
            projectile,
            lane: if near {
                ProjectileLane::Near
            } else {
                ProjectileLane::Far
            },
        }
    }

    pub fn resolve_impact(&self, projectile: u64, material: u16) -> ImpactProduct {
        ImpactProduct {
            projectile,
            material,
        }
    }
}
