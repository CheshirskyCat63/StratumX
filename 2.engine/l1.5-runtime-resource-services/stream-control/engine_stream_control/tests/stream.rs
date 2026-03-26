use engine_stream_control::{StreamClass, StreamController, StreamRequest, PAGE_HARD_CAP_BYTES};

#[test]
fn stream_controller_prioritizes_immediate_requests() {
    let mut controller = StreamController::default();
    controller
        .activate(StreamRequest {
            id: 2,
            class: StreamClass::Predicted,
            bytes: 1024,
            locality_score: 55,
            chunk_span: 1,
        })
        .expect("predicted queue");
    controller
        .activate(StreamRequest {
            id: 1,
            class: StreamClass::Immediate,
            bytes: 1024,
            locality_score: 0,
            chunk_span: 1,
        })
        .expect("immediate queue");

    assert_eq!(controller.schedule_next().expect("first").id, 1);
}

#[test]
fn stream_controller_enforces_page_caps() {
    let mut controller = StreamController::default();
    let result = controller.activate(StreamRequest {
        id: 3,
        class: StreamClass::Immediate,
        bytes: PAGE_HARD_CAP_BYTES + 1,
        locality_score: 0,
        chunk_span: 1,
    });
    assert!(result.is_err());
}
