use engine_net_latency::{InputSample, LatencyService};

#[test]
fn latency_rewind_window_is_bounded() {
    let service = LatencyService::default();
    assert!(service.validate_rewind_window(600).is_err());
}

#[test]
fn latency_prediction_advances_tick() {
    let mut service = LatencyService::default();
    service
        .push_input(InputSample {
            tick: 10,
            payload: 1,
        })
        .unwrap();
    assert!(service.predict_tick(10, 100) > 10);
}
