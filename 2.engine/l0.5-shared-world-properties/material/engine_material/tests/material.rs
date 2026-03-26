use engine_material::{
    MaterialDescriptor, MaterialId, MaterialLookup, PropertyDomain, ReactionRow, ResponseFlags,
    ResponseProfileId, MAX_DESCRIPTOR_CACHE_ROWS,
};
use smallvec::smallvec;

fn fallback_descriptor() -> MaterialDescriptor {
    MaterialDescriptor {
        material_id: MaterialId(0),
        class_id: 1,
        domains: smallvec![PropertyDomain::Physical],
        response_profile: ResponseProfileId(0),
    }
}

fn default_reaction() -> ReactionRow {
    ReactionRow {
        profile: ResponseProfileId(0),
        friction: 0.8,
        restitution: 0.1,
        flags: ResponseFlags::ABSORBENT,
    }
}

#[test]
fn lookup_uses_single_step_fallbacks() {
    let mut lookup = MaterialLookup::new(fallback_descriptor(), default_reaction());
    lookup
        .register_descriptor(MaterialDescriptor {
            material_id: MaterialId(2),
            class_id: 7,
            domains: smallvec![PropertyDomain::Thermal, PropertyDomain::Appearance],
            response_profile: ResponseProfileId(3),
        })
        .expect("register descriptor");

    let found = lookup.resolve_descriptor(MaterialId(2));
    assert_eq!(found.class_id, 7);

    let fallback = lookup.resolve_descriptor(MaterialId(99));
    assert_eq!(fallback.material_id, MaterialId(0));

    assert_eq!(
        lookup.resolve_reaction(ResponseProfileId(88)).profile,
        ResponseProfileId(0)
    );
}

#[test]
fn registration_respects_cache_ceiling() {
    let mut lookup = MaterialLookup::new(fallback_descriptor(), default_reaction());
    let result = lookup.register_descriptor(MaterialDescriptor {
        material_id: MaterialId(MAX_DESCRIPTOR_CACHE_ROWS as u32),
        class_id: 2,
        domains: smallvec![PropertyDomain::Fluid],
        response_profile: ResponseProfileId(1),
    });
    assert!(result.is_err());
}
