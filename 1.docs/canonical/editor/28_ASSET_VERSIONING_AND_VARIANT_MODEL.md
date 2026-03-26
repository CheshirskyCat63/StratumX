# Editor Asset Versioning and Variant Model

## Purpose

This document defines the editor-wide asset versioning and variant model: asset identity, versioning, variants, lineage, conversion/reimport compatibility, and packaging posture.

## Scope

The editor package (`L8` through `L11`) consumes asset versioning and variant information from tooling but does not own asset authority.

## Asset Identity

### Identity Structure
```rust
struct AssetIdentity {
    id: AssetId,
    path: AssetPath,
    type: AssetType,
    version: AssetVersion,
}

struct AssetId(Uuid);

struct AssetPath {
    package: PackageId,
    path: PathBuf,
}
```

### Identity Discipline
- Asset identity is owned by `L6.0` authority-core
- Asset identity is immutable (ID never changes)
- Asset path may change (rename, move)
- Asset type is immutable (cannot change type after creation)

## Asset Versioning

### Versioning Structure
```rust
struct AssetVersion {
    major: u32,
    minor: u32,
    patch: u32,
}
```

### Versioning Discipline
- Asset version is owned by `L6.1` asset-compiler
- Asset version is incremented on recompilation
- Asset version is used for dependency tracking
- Asset version is used for cache invalidation

### Versioning Examples
- `1.0.0` → `1.1.0`: Asset recompiled with new compiler (backward-compatible)
- `1.1.0` → `2.0.0`: Asset recompiled with breaking changes (not backward-compatible)
- `2.0.0` → `2.0.1`: Asset recompiled with bug fix (backward-compatible)

## Asset Variants

### Variant Structure
```rust
struct AssetVariant {
    id: VariantId,
    base_asset: AssetId,
    variant_type: VariantType,
    properties: VariantProperties,
}

enum VariantType {
    LOD(u32),              // Level of detail (0 = highest, 1 = medium, 2 = low)
    Platform(PlatformId),  // Platform-specific (PC, Console, Mobile)
    Quality(QualityId),    // Quality-specific (High, Medium, Low)
    Custom(String),        // Custom variant
}
```

### Variant Discipline
- Asset variants are owned by `L6.1` asset-compiler
- Asset variants share the same base asset ID
- Asset variants may have different properties (resolution, compression, etc.)
- Asset variants are selected at runtime based on platform/quality/LOD

### Variant Examples
- Texture variants: High-res (4K), Medium-res (2K), Low-res (1K)
- Model variants: LOD0 (high-poly), LOD1 (medium-poly), LOD2 (low-poly)
- Audio variants: High-quality (uncompressed), Medium-quality (compressed), Low-quality (streaming)

## Asset Lineage

### Lineage Structure
```rust
struct AssetLineage {
    asset: AssetId,
    source: Option<AssetSource>,
    dependencies: Vec<AssetId>,
    dependents: Vec<AssetId>,
}

enum AssetSource {
    Imported(PathBuf),
    Generated(GeneratorId),
    Derived(AssetId),
}
```

### Lineage Discipline
- Asset lineage is owned by `L6.1` asset-compiler
- Asset lineage tracks source (imported file, generator, derived from another asset)
- Asset lineage tracks dependencies (assets this asset depends on)
- Asset lineage tracks dependents (assets that depend on this asset)

### Lineage Examples
- Imported texture: Source = imported file, Dependencies = none, Dependents = materials
- Generated material: Source = material generator, Dependencies = textures, Dependents = meshes
- Derived LOD: Source = derived from LOD0, Dependencies = LOD0, Dependents = none

## Conversion and Reimport Compatibility

### Conversion Discipline
- Asset conversion is owned by `L6.1` asset-compiler
- Asset conversion may change asset version
- Asset conversion may change asset properties
- Asset conversion must preserve asset identity (ID)

### Reimport Discipline
- Asset reimport is owned by `L6.1` asset-compiler
- Asset reimport may change asset version
- Asset reimport may change asset properties
- Asset reimport must preserve asset identity (ID)
- Asset reimport must preserve asset dependencies (if possible)

### Compatibility Posture
- Asset conversion must be backward-compatible (old dependents still work)
- Asset reimport must be backward-compatible (old dependents still work)
- If backward compatibility is not possible, show warning dialog

## Packaging Posture

### Packaging Discipline
- Asset packaging is owned by `L7.1` release-packager
- Asset packaging selects appropriate variants for target platform/quality
- Asset packaging bundles assets into packages
- Asset packaging compresses assets (if beneficial)

### Packaging Examples
- PC package: High-quality variants, uncompressed
- Console package: Medium-quality variants, compressed
- Mobile package: Low-quality variants, highly compressed

## Editor Integration

### Editor Consumption
- The editor consumes asset identity from `L6.0` authority-core (read-only)
- The editor consumes asset version from `L6.1` asset-compiler (read-only)
- The editor consumes asset variants from `L6.1` asset-compiler (read-only)
- The editor consumes asset lineage from `L6.1` asset-compiler (read-only)

### Editor UI
- Show asset identity in content browser (ID, path, type)
- Show asset version in inspector (version, last compiled)
- Show asset variants in inspector (LOD, platform, quality)
- Show asset lineage in inspector (source, dependencies, dependents)

### Editor Operations
- Trigger asset reimport via `L6.1` asset-compiler
- Trigger asset conversion via `L6.1` asset-compiler
- Trigger asset packaging via `L7.1` release-packager

## Forbidden Patterns

The editor must NOT:
- Own asset identity (consumed from `L6.0`)
- Own asset version (consumed from `L6.1`)
- Own asset variants (consumed from `L6.1`)
- Own asset lineage (consumed from `L6.1`)
- Modify asset identity directly
- Modify asset version directly
- Modify asset variants directly
- Modify asset lineage directly

## Version

This document is part of the `SX-CANON/1.0.6/STACK-v12` editor package.
