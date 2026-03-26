use engine_imaging::compute_visibility;

#[test]
fn imaging_rejects_visibility_older_than_one_frame() {
    assert!(compute_visibility(2).is_none());
}
