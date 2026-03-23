# Libraries

## External Library Baseline

| Library | Role in `engine_content` |
|---|---|
| `walkdir` | File and directory traversal for content ingest. |
| `image` | Image asset decoding where required. |
| `gltf` | Structured scene/content ingest where required. |
| `serde` | Serialization for content descriptors and metadata. |
| `thiserror` | Typed content pipeline errors. |
| `smallvec` | Low-allocation pipeline metadata. |
| `tracing` | Content diagnostics. |

## Fit

Each listed dependency serves the primary role of `engine_content` and stays aligned to its pressure axis.
