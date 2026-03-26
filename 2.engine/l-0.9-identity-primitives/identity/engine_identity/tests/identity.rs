use engine_identity::{
    validate_domain, validate_generation, ComponentId, ComponentKey, EntityId, EntityKey,
    Generation, IdentityDomain, IdentityError, ReuseDelay,
};
use slotmap::Key;

#[test]
fn entity_id_exposes_key_generation_and_domain() {
    let key = EntityKey::null();
    let generation = Generation::INITIAL;
    let entity = EntityId::new(key, generation);

    assert_eq!(entity.key(), key);
    assert_eq!(entity.generation(), generation);
    assert_eq!(EntityId::DOMAIN, IdentityDomain::Entity);
    assert_eq!(entity.validate_domain(IdentityDomain::Entity), Ok(()));
}

#[test]
fn component_id_exposes_key_generation_and_domain() {
    let key = ComponentKey::null();
    let generation = Generation::INITIAL.next();
    let component = ComponentId::new(key, generation);

    assert_eq!(component.key(), key);
    assert_eq!(component.generation(), generation);
    assert_eq!(ComponentId::DOMAIN, IdentityDomain::Component);
    assert_eq!(component.validate_domain(IdentityDomain::Component), Ok(()));
}

#[test]
fn generation_validation_rejects_stale_identity() {
    let provided = Generation(7);
    let current = Generation(8);

    let error = validate_generation(provided, current).expect_err("must reject stale identity");
    assert_eq!(error, IdentityError::StaleIdentity { provided, current });
}

#[test]
fn domain_validation_rejects_cross_domain_usage() {
    let error = validate_domain(IdentityDomain::Entity, IdentityDomain::Component)
        .expect_err("domain mismatch must be rejected");

    assert_eq!(
        error,
        IdentityError::InvalidDomain {
            expected: IdentityDomain::Entity,
            found: IdentityDomain::Component,
        }
    );
}

#[test]
fn generation_wrap_skips_zero() {
    let wrapped = Generation(u32::MAX).next();
    assert_eq!(wrapped, Generation::INITIAL);
}

#[test]
fn reuse_delay_enforces_reuse_lag() {
    let delay = ReuseDelay::new(2);

    assert!(!delay.can_reuse(11, 10));
    assert!(delay.can_reuse(12, 10));
}
