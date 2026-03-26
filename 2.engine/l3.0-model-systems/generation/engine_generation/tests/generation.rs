use engine_generation::{package, transform, GenerationProduct};

#[test]
fn generation_pipeline_transforms_then_packages() {
    let product = transform(GenerationProduct {
        body: " hi ".into(),
    });
    let packaged = package(product);
    assert_eq!(packaged.payload, "hi");
}
