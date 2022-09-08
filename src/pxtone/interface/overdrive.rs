use std::{fmt, ops::Deref};

use crate::util::{BoxOrMut, BoxOrRef};

/// Wrapper for an f32 representing an overdrive cut value, 0.5 to 0.999 (inclusive).
#[derive(Clone, Copy, Debug)]
pub struct OverDCut(f32);

impl OverDCut {
    /// Create a `OverDCut` from a normal f32.
    ///
    /// The value is clamped if outside 0.5..=0.999.
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.5, 0.999))
    }
}

impl Deref for OverDCut {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper for an f32 representing an overdrive amp value, 0.1 to 8.0 (inclusive).
#[derive(Clone, Copy, Debug)]
pub struct OverDAmp(f32);

impl OverDAmp {
    /// Create a `OverDAmp` from a normal f32.
    ///
    /// The value is clamped if outside 0.1..=8.0.
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.1, 8.0))
    }
}

impl Deref for OverDAmp {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait OverDrive {
    fn group(&self) -> u8;
    fn set_group(&mut self, group: u8);

    fn cut(&self) -> OverDCut;
    fn set_cut(&mut self, cut: OverDCut);

    fn amp(&self) -> OverDAmp;
    fn set_amp(&mut self, amp: OverDAmp);
}

pub trait OverDrives {
    type O: OverDrive;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::O>> + 'a>;
}

#[derive(Debug)]
pub struct AddOverDriveError {
    pub group: u8,
    pub cut: OverDCut,
    pub amp: OverDAmp,
}

impl fmt::Display for AddOverDriveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Text")
    }
}

impl std::error::Error for AddOverDriveError {}

pub trait OverDrivesMut: OverDrives {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::O>> + 'a>;

    fn add(&mut self, group: u8, cut: OverDCut, amp: OverDAmp) -> Result<(), AddOverDriveError>;

    fn remove(&mut self, index: usize) -> bool;
}
