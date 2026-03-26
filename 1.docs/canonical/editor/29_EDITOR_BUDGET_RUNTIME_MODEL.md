# Editor Budget Runtime Model

## Purpose

This document defines the editor-wide budget runtime model: budgets for all surfaces, hot vs cold surface costs, and refresh/invalidation budget posture.

## Scope

The editor package (`L8` through `L11`) enforces resource budgets for all surfaces to prevent unbounded resource consumption.

## Budget Categories

### Memory Budget
- Viewport render targets
- Panel UI state
- Preview caches
- Indexing caches
- Diagnostics caches
- Collaboration caches
- Undo/redo history

### CPU Budget
- Viewport rendering
- Panel UI updates
- Preview generation
- Import/export processing
- Build processing
- Diagnostics processing
- Indexing processing
- Collaboration processing

### GPU Budget
- Viewport rendering
- Preview rendering
- Overlay rendering
- Gizmo rendering

### Disk Budget
- Autosave artifacts
- Recovery artifacts
- Preview artifacts
- Build artifacts
- Export artifacts
- Temporary artifacts

### Network Budget
- Collaboration traffic
- Package market traffic
- Assistant traffic

## Surface Budgets

### Viewport Budget
```rust
struct ViewportBudget {
    memory: MemoryBudget,
    cpu: CpuBudget,
    gpu: GpuBudget,
}

struct MemoryBudget {
    render_targets: usize,  // e.g., 256 MB per viewport
    preview_cache: usize,   // e.g., 512 MB shared across viewports
}

struct CpuBudget {
    frame_time: Duration,   // e.g., 16 ms (60 FPS)
    refresh_time: Duration, // e.g., 100 ms (10 Hz)
}

struct GpuBudget {
    draw_calls: usize,      // e.g., 10,000 draw calls per frame
    triangles: usize,       // e.g., 10 million triangles per frame
}
```

### Panel Budget
```rust
struct PanelBudget {
    memory: MemoryBudget,
    cpu: CpuBudget,
}

struct MemoryBudget {
    ui_state: usize,        // e.g., 16 MB per panel
    cache: usize,           // e.g., 64 MB per panel
}

struct CpuBudget {
    update_time: Duration,  // e.g., 16 ms (60 FPS)
    refresh_time: Duration, // e.g., 100 ms (10 Hz)
}
```

### Preview Budget
```rust
struct PreviewBudget {
    memory: MemoryBudget,
    cpu: CpuBudget,
    disk: DiskBudget,
}

struct MemoryBudget {
    cache: usize,           // e.g., 512 MB shared across all previews
}

struct CpuBudget {
    generation_time: Duration, // e.g., 1 second per preview
}

struct DiskBudget {
    artifacts: usize,       // e.g., 1 GB shared across all previews
}
```

### Import/Export Budget
```rust
struct ImportExportBudget {
    memory: MemoryBudget,
    cpu: CpuBudget,
    disk: DiskBudget,
}

struct MemoryBudget {
    buffer: usize,          // e.g., 256 MB per import/export job
}

struct CpuBudget {
    processing_time: Duration, // e.g., 10 seconds per asset
}

struct DiskBudget {
    artifacts: usize,       // e.g., 10 GB shared across all import/export jobs
}
```

### Build Budget
```rust
struct BuildBudget {
    memory: MemoryBudget,
    cpu: CpuBudget,
    disk: DiskBudget,
}

struct MemoryBudget {
    buffer: usize,          // e.g., 1 GB per build job
}

struct CpuBudget {
    processing_time: Duration, // e.g., 10 minutes per build
}

struct DiskBudget {
    artifacts: usize,       // e.g., 50 GB shared across all build jobs
}
```

### Diagnostics Budget
```rust
struct DiagnosticsBudget {
    memory: MemoryBudget,
    cpu: CpuBudget,
}

struct MemoryBudget {
    cache: usize,           // e.g., 128 MB shared across all diagnostics
}

struct CpuBudget {
    processing_time: Duration, // e.g., 1 second per diagnostics run
}
```

### Indexing Budget
```rust
struct IndexingBudget {
    memory: MemoryBudget,
    cpu: CpuBudget,
}

struct MemoryBudget {
    cache: usize,           // e.g., 256 MB shared across all indexing
}

struct CpuBudget {
    processing_time: Duration, // e.g., 10 seconds per indexing run
}
```

### Collaboration Budget
```rust
struct CollaborationBudget {
    memory: MemoryBudget,
    cpu: CpuBudget,
    network: NetworkBudget,
}

struct MemoryBudget {
    cache: usize,           // e.g., 128 MB shared across all collaboration
}

struct CpuBudget {
    processing_time: Duration, // e.g., 100 ms per collaboration message
}

struct NetworkBudget {
    bandwidth: usize,       // e.g., 10 MB/s shared across all collaboration
}
```

### Assistant Budget
```rust
struct AssistantBudget {
    memory: MemoryBudget,
    cpu: CpuBudget,
    network: NetworkBudget,
}

struct MemoryBudget {
    cache: usize,           // e.g., 64 MB shared across all assistant
}

struct CpuBudget {
    processing_time: Duration, // e.g., 5 seconds per assistant request
}

struct NetworkBudget {
    bandwidth: usize,       // e.g., 1 MB/s shared across all assistant
}
```

## Hot vs Cold Surface Costs

### Hot Surface
- Visible and actively consuming resources
- Full budget allocation
- Immediate refresh on invalidation

### Warm Surface
- Not visible but retains some resources for fast reactivation
- Reduced budget allocation (e.g., 25% of full budget)
- Deferred refresh on invalidation

### Cold Surface
- Not visible and has released all resources
- Zero budget allocation
- No refresh on invalidation (refresh on reactivation)

## Refresh and Invalidation Budget Posture

### Refresh Discipline
- Refresh is triggered by invalidation
- Refresh is bounded by budget
- Refresh is deferred if budget is exceeded
- Refresh is cancelled if surface is deactivated

### Invalidation Discipline
- Invalidation is triggered by mutation
- Invalidation is batched (multiple invalidations in one refresh)
- Invalidation is prioritized (hot surfaces > warm surfaces > cold surfaces)
- Invalidation is cancelled if surface is deactivated

### Budget Enforcement
- If budget is exceeded, show warning dialog
- If budget is exceeded, defer refresh until budget is available
- If budget is exceeded, reduce quality (e.g., lower LOD, lower resolution)
- If budget is exceeded, deactivate cold surfaces

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
