# Resource Discipline

## CPU
- inactive panels do not tick
- inactive suites do not run heavy rebuilds
- hidden overlays do not re-render expensive data

## GPU
- only visible viewport surfaces keep GPU-backed presentation active
- preview textures and thumbnails are pooled and evicted

## RAM
- browsers and inspectors do not mirror bulk domain truth
- thumbnail caches, search indices, and preview buffers are bounded

## Disk
- autosave, cache, import staging, and derived outputs are separated and bounded

## Law
Editor comfort features may not silently become unbounded memory or disk sinks.
