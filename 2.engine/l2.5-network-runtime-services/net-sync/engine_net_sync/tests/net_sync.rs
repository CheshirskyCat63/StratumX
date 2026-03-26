use engine_net_sync::{InterestEntry, NetSyncService};

#[test]
fn sync_ack_window_is_bounded() {
    let service = NetSyncService::default();
    assert!(service.ack(1, 257).is_err());
}

#[test]
fn sync_registers_region_first_interest() {
    let mut service = NetSyncService::default();
    service
        .register_interest(InterestEntry {
            connection_id: 1,
            region: 12,
        })
        .unwrap();
}
