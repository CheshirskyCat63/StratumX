pub type EngineCoreResult<T> = Result<T, EngineCoreError>;

#[derive(Debug, PartialEq, Eq)]
pub enum EngineCoreError {
    InvalidDescriptor(&'static str),
    FeatureConflict(&'static str),
}

impl core::fmt::Display for EngineCoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidDescriptor(message) => write!(f, "invalid descriptor: {message}"),
            Self::FeatureConflict(message) => write!(f, "feature profile conflict: {message}"),
        }
    }
}

impl std::error::Error for EngineCoreError {}
