use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerationContext {
    pub seed: u64,
    pub topic: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerationProduct {
    pub body: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackagedOutput {
    pub mime: &'static str,
    pub payload: String,
}

pub fn expand_context(seed: u64, topic: impl Into<String>) -> GenerationContext {
    GenerationContext {
        seed,
        topic: topic.into(),
    }
}

pub fn transform(product: GenerationProduct) -> GenerationProduct {
    GenerationProduct {
        body: product.body.trim().to_string(),
    }
}

pub fn package(product: GenerationProduct) -> PackagedOutput {
    PackagedOutput {
        mime: "text/plain",
        payload: product.body,
    }
}
