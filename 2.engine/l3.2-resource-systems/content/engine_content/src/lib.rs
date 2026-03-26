use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IngestProduct {
    pub source_path: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransformProduct {
    pub artifact_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataProduct {
    pub key: String,
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimePackProduct {
    pub pack_id: String,
    pub seekable: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamingChunkProduct {
    pub chunk_id: String,
    pub region: u32,
}

pub fn ingest(path: impl Into<String>) -> IngestProduct {
    IngestProduct {
        source_path: path.into(),
    }
}
pub fn transform(artifact_id: impl Into<String>) -> TransformProduct {
    TransformProduct {
        artifact_id: artifact_id.into(),
    }
}
pub fn metadata(key: impl Into<String>, value: impl Into<String>) -> MetadataProduct {
    MetadataProduct {
        key: key.into(),
        value: value.into(),
    }
}
pub fn runtime_pack(pack_id: impl Into<String>) -> RuntimePackProduct {
    RuntimePackProduct {
        pack_id: pack_id.into(),
        seekable: true,
    }
}
pub fn streaming_chunk(chunk_id: impl Into<String>, region: u32) -> StreamingChunkProduct {
    StreamingChunkProduct {
        chunk_id: chunk_id.into(),
        region,
    }
}
