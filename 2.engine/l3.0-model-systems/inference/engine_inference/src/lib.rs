use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InferenceContext {
    pub world_tick: u32,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuredOutput {
    pub label: String,
    pub confidence_basis_points: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InferenceBudget {
    pub max_tokens: u16,
    pub max_millis: u16,
}

pub trait ModelAdapter {
    fn infer(&self, context: &InferenceContext, budget: InferenceBudget) -> StructuredOutput;
}

pub fn assemble_context(world_tick: u32, prompt: impl Into<String>) -> InferenceContext {
    InferenceContext {
        world_tick,
        prompt: prompt.into(),
    }
}

pub fn run_inference(
    adapter: &dyn ModelAdapter,
    context: &InferenceContext,
    budget: InferenceBudget,
) -> Option<StructuredOutput> {
    if budget.max_tokens == 0 || budget.max_millis == 0 {
        return None;
    }
    Some(adapter.infer(context, budget))
}
