use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropagationProduct {
    pub distance_m: f32,
    pub attenuation_db: f32,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReflectionProduct {
    pub bounce_count: u8,
    pub occluded: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeBuffer {
    pub samples: u32,
}

pub fn propagate(distance_m: f32) -> PropagationProduct {
    PropagationProduct {
        distance_m,
        attenuation_db: -(distance_m / 10.0),
    }
}

pub fn reflect_and_occlude(bounce_count: u8, occluded: bool) -> ReflectionProduct {
    ReflectionProduct {
        bounce_count,
        occluded,
    }
}

pub fn stage_runtime_buffer(samples: u32) -> RuntimeBuffer {
    RuntimeBuffer { samples }
}
