use engine_net_transport::{PacketDescriptor, PacketLane, TransportService, TransportSession};

#[test]
fn transport_rejects_unauthenticated_sessions() {
    let mut service = TransportService::default();
    let err = service.open_session(TransportSession {
        id: 1,
        authenticated: false,
    });
    assert!(err.is_err());
}

#[test]
fn transport_validates_lane_size() {
    let service = TransportService::default();
    let err = service.publish(&PacketDescriptor {
        lane: PacketLane::Input,
        bytes: 1201,
    });
    assert!(err.is_err());
}
