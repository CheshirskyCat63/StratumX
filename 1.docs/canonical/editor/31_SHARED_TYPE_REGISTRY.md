# Editor Shared Type Registry

## Purpose

This document defines the editor-wide shared type registry: viewport IDs, panel IDs, shell IDs, suite IDs, service IDs, mode IDs, overlay/gizmo types, request/result classes, diagnostics and collaboration classes.

## Scope

The editor package (`L8` through `L11`) uses typed identifiers and message classes for all cross-surface communication.

## Viewport IDs

```rust
enum ViewportId {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    Custom(u32),
}

enum ViewportMode {
    Perspective3D,
    Orthographic2D,
    Top,
    Front,
    Side,
    Custom(String),
}
```

## Panel IDs

```rust
enum PanelId {
    // L8.2 Outliner
    WorldOutliner,
    EntityBrowser,
    
    // L8.3 Content Browser
    ContentBrowser,
    PackageBrowser,
    
    // L8.4 Inspector
    PropertyInspector,
    DetailsPanel,
    
    // L8.5 Diagnostics
    DiagnosticsPanel,
    ValidationPanel,
    
    // L8.9 Assistant
    AssistantPanel,
    
    // L8.10 Diagnostics Surface
    DiagnosticsSurface,
    
    // L8.11 Build/Release
    BuildPanel,
    ReleasePanel,
    
    // L11 Collaboration
    CollaborationPanel,
    ReviewPanel,
    AnnotationPanel,
    PlaytestPanel,
    
    Custom(String),
}
```

## Shell IDs

```rust
enum ShellId {
    MainWindow,
    SecondaryWindow(u32),
}

enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Center,
    Floating,
}
```

## Suite IDs

```rust
enum SuiteId {
    // L9.0 World/Scene
    WorldSuite,
    SceneSuite,
    
    // L9.1 Terrain/Material/Environment
    TerrainSuite,
    MaterialSuite,
    EnvironmentSuite,
    
    // L9.2 Destruction/Simulation
    DestructionSuite,
    SimulationSuite,
    
    // L9.3 Animation/Audio/UI
    AnimationSuite,
    AudioSuite,
    UISuite,
    
    Custom(String),
}
```

## Service IDs

```rust
enum ServiceId {
    // L10.0 Project Bootstrap
    ProjectBootstrap,
    
    // L10.1 Import/Export
    ImportExport,
    
    // L10.2 Graph Authoring
    GraphAuthoring,
    
    // L10.3 Automation
    Automation,
    
    // L10.4 Script/Hot-Reload
    ScriptHotReload,
    
    // L10.5 Plugin/Extension
    PluginExtension,
    
    // L10.6 Template/Preset
    TemplatePreset,
    
    // L10.7 Package/Market
    PackageMarket,
    
    Custom(String),
}
```

## Mode IDs

```rust
enum ModeId {
    Select,
    Move,
    Rotate,
    Scale,
    Paint,
    Sculpt,
    Place,
    Measure,
    Custom(String),
}

enum ToolContext {
    World,
    Terrain,
    Material,
    Animation,
    Audio,
    UI,
    Custom(String),
}
```

## Overlay and Gizmo Types

```rust
enum OverlayType {
    Grid,
    Axis,
    Bounds,
    Wireframe,
    Normals,
    Tangents,
    Colliders,
    Lights,
    Cameras,
    Audio,
    Custom(String),
}

enum GizmoType {
    TranslationGizmo,
    RotationGizmo,
    ScaleGizmo,
    BoundsGizmo,
    PathGizmo,
    Custom(String),
}
```

## Request/Result Classes

### Preview Request/Result
```rust
struct PreviewRequest {
    asset: AssetId,
    viewport: ViewportId,
    mode: PreviewMode,
}

struct PreviewResult {
    asset: AssetId,
    viewport: ViewportId,
    status: PreviewStatus,
    error: Option<String>,
}

enum PreviewMode {
    Static,
    Animated,
    Interactive,
}

enum PreviewStatus {
    Success,
    Failed,
    Cancelled,
}
```

### Build Request/Result
```rust
struct BuildRequest {
    target: BuildTarget,
    configuration: BuildConfiguration,
}

struct BuildResult {
    target: BuildTarget,
    status: BuildStatus,
    artifacts: Vec<ArtifactPath>,
    errors: Vec<BuildError>,
}

enum BuildTarget {
    Development,
    Shipping,
    Custom(String),
}

enum BuildStatus {
    Success,
    Failed,
    Cancelled,
}
```

### Import/Export Request/Result
```rust
struct ImportRequest {
    source: PathBuf,
    target: AssetId,
    options: ImportOptions,
}

struct ImportResult {
    source: PathBuf,
    target: AssetId,
    status: ImportStatus,
    error: Option<String>,
}

struct ExportRequest {
    source: AssetId,
    target: PathBuf,
    options: ExportOptions,
}

struct ExportResult {
    source: AssetId,
    target: PathBuf,
    status: ExportStatus,
    error: Option<String>,
}
```

## Diagnostics Classes

```rust
struct DiagnosticMessage {
    id: DiagnosticId,
    severity: DiagnosticSeverity,
    message: String,
    source: DiagnosticSource,
    location: Option<DiagnosticLocation>,
}

enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

enum DiagnosticSource {
    Compiler,
    Validator,
    Linter,
    Custom(String),
}

struct DiagnosticLocation {
    asset: AssetId,
    line: u32,
    column: u32,
}
```

## Collaboration Classes

```rust
struct CollaborationSession {
    id: SessionId,
    participants: Vec<ParticipantId>,
    status: SessionStatus,
}

enum SessionStatus {
    Active,
    Paused,
    Ended,
}

struct ReviewAnnotation {
    id: AnnotationId,
    author: ParticipantId,
    target: AnnotationTarget,
    message: String,
    timestamp: Timestamp,
}

enum AnnotationTarget {
    Entity(EntityId),
    Asset(AssetId),
    Location(WorldLocation),
}

struct PlaytestSession {
    id: SessionId,
    participants: Vec<ParticipantId>,
    recording: Option<RecordingId>,
    status: SessionStatus,
}
```

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
