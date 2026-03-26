use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneExtraction {
    pub visible_regions: Vec<u32>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisibilitySet {
    pub stale_frames: u8,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightingSet {
    pub probe_count: u16,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageOutput {
    pub width: u16,
    pub height: u16,
}

pub fn extract_scene(regions: Vec<u32>) -> SceneExtraction {
    SceneExtraction {
        visible_regions: regions,
    }
}
pub fn compute_visibility(stale_frames: u8) -> Option<VisibilitySet> {
    (stale_frames <= 1).then_some(VisibilitySet { stale_frames })
}
pub fn compute_lighting(probe_count: u16) -> LightingSet {
    LightingSet { probe_count }
}
pub fn synthesize_image(width: u16, height: u16) -> ImageOutput {
    ImageOutput { width, height }
}
