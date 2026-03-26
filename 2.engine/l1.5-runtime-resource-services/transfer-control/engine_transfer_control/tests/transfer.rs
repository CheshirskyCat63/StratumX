use engine_transfer_control::{
    DecodeClass, TransferBatch, TransferController, TransferDescriptor, TransferFence,
};
use smallvec::smallvec;

#[test]
fn transfer_rejects_raw_asset_decode() {
    let mut transfer = TransferController::default();
    let batch = TransferBatch {
        transfers: smallvec![TransferDescriptor {
            id: 1,
            decode_class: DecodeClass::RawAuthoredAsset,
            bytes: 1024,
        }],
    };
    assert!(transfer.submit_batch(&batch).is_err());
}

#[test]
fn transfer_releases_after_fence_visibility() {
    let mut transfer = TransferController::default();
    let batch = TransferBatch {
        transfers: smallvec![TransferDescriptor {
            id: 5,
            decode_class: DecodeClass::TextureTranscodeReady,
            bytes: 1024,
        }],
    };

    transfer.submit_batch(&batch).expect("submit batch");
    transfer
        .mark_fence_visible(
            TransferFence {
                id: 2,
                visible: true,
            },
            &[5],
        )
        .expect("mark fence visible");
    let released = transfer.release_ready();
    assert_eq!(released.as_slice(), &[5]);
}
