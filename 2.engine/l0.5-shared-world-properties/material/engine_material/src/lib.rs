use bitflags::bitflags;
use engine_core::{EngineCoreError, EngineCoreResult};
use smallvec::SmallVec;

pub const MAX_DESCRIPTOR_CACHE_ROWS: usize = 4096;
pub const MAX_REACTION_CACHE_ROWS: usize = 4096;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct MaterialId(pub u32);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ResponseProfileId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PropertyDomain {
    Physical,
    Thermal,
    Fluid,
    Structural,
    Acoustic,
    Appearance,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub struct ResponseFlags: u16 {
        const BOUNCY = 0b0001;
        const ABSORBENT = 0b0010;
        const CONDUCTIVE = 0b0100;
        const INSULATING = 0b1000;
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaterialDescriptor {
    pub material_id: MaterialId,
    pub class_id: u16,
    pub domains: SmallVec<[PropertyDomain; 6]>,
    pub response_profile: ResponseProfileId,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReactionRow {
    pub profile: ResponseProfileId,
    pub friction: f32,
    pub restitution: f32,
    pub flags: ResponseFlags,
}

#[derive(Debug)]
pub struct MaterialLookup {
    descriptor_rows: Vec<Option<MaterialDescriptor>>,
    reaction_rows: Vec<Option<ReactionRow>>,
    fallback_descriptor: MaterialDescriptor,
    default_reaction: ReactionRow,
}

impl MaterialLookup {
    pub fn new(fallback_descriptor: MaterialDescriptor, default_reaction: ReactionRow) -> Self {
        Self {
            descriptor_rows: Vec::new(),
            reaction_rows: Vec::new(),
            fallback_descriptor,
            default_reaction,
        }
    }

    pub fn register_descriptor(&mut self, descriptor: MaterialDescriptor) -> EngineCoreResult<()> {
        let idx = descriptor.material_id.0 as usize;
        self.ensure_descriptor_capacity(idx)?;
        self.descriptor_rows[idx] = Some(descriptor);
        Ok(())
    }

    pub fn register_reaction(&mut self, row: ReactionRow) -> EngineCoreResult<()> {
        let idx = row.profile.0 as usize;
        self.ensure_reaction_capacity(idx)?;
        self.reaction_rows[idx] = Some(row);
        Ok(())
    }

    pub fn resolve_descriptor(&self, id: MaterialId) -> &MaterialDescriptor {
        self.descriptor_rows
            .get(id.0 as usize)
            .and_then(Option::as_ref)
            .unwrap_or(&self.fallback_descriptor)
    }

    pub fn resolve_reaction(&self, profile: ResponseProfileId) -> &ReactionRow {
        self.reaction_rows
            .get(profile.0 as usize)
            .and_then(Option::as_ref)
            .unwrap_or(&self.default_reaction)
    }

    fn ensure_descriptor_capacity(&mut self, idx: usize) -> EngineCoreResult<()> {
        if idx >= MAX_DESCRIPTOR_CACHE_ROWS {
            return Err(EngineCoreError::InvalidDescriptor(
                "descriptor row exceeds canonical cache ceiling",
            ));
        }
        if self.descriptor_rows.len() <= idx {
            self.descriptor_rows.resize(idx + 1, None);
        }
        Ok(())
    }

    fn ensure_reaction_capacity(&mut self, idx: usize) -> EngineCoreResult<()> {
        if idx >= MAX_REACTION_CACHE_ROWS {
            return Err(EngineCoreError::InvalidDescriptor(
                "reaction row exceeds canonical cache ceiling",
            ));
        }
        if self.reaction_rows.len() <= idx {
            self.reaction_rows.resize(idx + 1, None);
        }
        Ok(())
    }
}
