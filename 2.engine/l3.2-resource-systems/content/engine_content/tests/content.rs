use engine_content::runtime_pack;

#[test]
fn runtime_pack_products_are_seekable() {
    assert!(runtime_pack("pack-a").seekable);
}
