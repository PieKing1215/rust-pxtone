use std::{fmt::Debug, path::PathBuf};

/// Trait that covers reading/writing the project
pub trait PxToneServiceIO {
    type Error: Debug;

    fn read_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;

    fn write_file(&mut self, path: impl Into<PathBuf>) -> Result<Vec<u8>, Self::Error>;
}
