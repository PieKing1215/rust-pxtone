use std::{path::{PathBuf}};

/// Trait that covers reading/writing the project
pub trait PxToneServiceIO {
    type Error;

    fn read_bytes(bytes: &[u8]) -> Result<Self, Self::Error> where Self: Sized;

    fn write_file(&mut self, path: impl Into<PathBuf>) -> Result<Vec<u8>, Self::Error>;
}