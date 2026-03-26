use engine_agents::{ActionIntent, AgentsFamily};

#[test]
fn agents_emit_bounded_action_intents() {
    let mut family = AgentsFamily::default();
    family
        .enqueue_intent(ActionIntent {
            agent: 4,
            action: "advance",
        })
        .unwrap();
    assert_eq!(family.intents().len(), 1);
}
