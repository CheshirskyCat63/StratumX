use engine_startup::{bootstrap, invalidate, NetworkRole, RuntimeProfile, StartupConfig};

#[test]
fn startup_rejects_invalid_profile_role_combo() {
    let cfg = StartupConfig {
        profile: RuntimeProfile::Headless20,
        role: NetworkRole::InteractiveHostAware,
    };
    assert!(bootstrap(&cfg).is_err());
}

#[test]
fn startup_exports_epoch_and_invalidation() {
    let cfg = StartupConfig {
        profile: RuntimeProfile::Interactive60,
        role: NetworkRole::Local,
    };
    let mut epoch = bootstrap(&cfg).unwrap();
    assert!(!epoch.invalidated);
    invalidate(&mut epoch);
    assert!(epoch.invalidated);
}
