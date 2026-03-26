/// Stable entity identity across engine layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

/// Simulation tick index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tick(pub u64);

/// Lightweight descriptor for component families.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentKind {
    pub name: String,
    pub version: u16,
}
