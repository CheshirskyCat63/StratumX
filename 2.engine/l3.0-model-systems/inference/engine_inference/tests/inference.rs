use engine_inference::{
    assemble_context, run_inference, InferenceBudget, InferenceContext, ModelAdapter,
    StructuredOutput,
};

struct EchoAdapter;
impl ModelAdapter for EchoAdapter {
    fn infer(&self, context: &InferenceContext, _budget: InferenceBudget) -> StructuredOutput {
        StructuredOutput {
            label: context.prompt.clone(),
            confidence_basis_points: 9000,
        }
    }
}

#[test]
fn inference_respects_budget_zero_guard() {
    let ctx = assemble_context(1, "move");
    assert!(run_inference(
        &EchoAdapter,
        &ctx,
        InferenceBudget {
            max_tokens: 0,
            max_millis: 10
        }
    )
    .is_none());
}
