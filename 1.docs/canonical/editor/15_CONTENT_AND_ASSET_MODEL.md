# Content and Asset Model

## Asset classes
- source assets
- imported/intermediate artifacts
- authored project assets
- preview artifacts
- cooked/release artifacts

## Product responsibilities
- browse and filter assets
- inspect metadata
- stage imports and reimports
- show dependencies and references
- surface health and validation

## Laws
- source and imported artifact classes remain distinct
- asset browsing views are read-mostly and index-backed
- import and conversion work belongs to services, not shell views
