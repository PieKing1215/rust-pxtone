use std::fmt;

use crate::{
    pxtone::util::ZeroToOneF32,
    util::{BoxOrMut, BoxOrRef},
};

#[derive(Clone, Copy, Debug)]
pub enum DelayUnit {
    Beat(f32),
    Measure(f32),
    Second(f32),
}

pub trait Delay {
    fn group(&self) -> u8;
    fn set_group(&mut self, group: u8);

    fn frequency(&self) -> DelayUnit;
    fn set_frequency(&mut self, frequency: DelayUnit);

    fn rate(&self) -> ZeroToOneF32;
    fn set_rate(&mut self, rate: ZeroToOneF32);
}

pub trait Delays {
    type D: Delay;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::D>> + 'a>;
}

#[derive(Debug)]
pub struct AddDelayError {
    pub group: u8,
    pub frequency: DelayUnit,
    pub rate: ZeroToOneF32,
}

impl fmt::Display for AddDelayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to add Delay(group={} frequency={:?} rate={:?})",
            self.group, self.frequency, self.rate
        )
    }
}

impl std::error::Error for AddDelayError {}

pub trait DelaysMut: Delays {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::D>> + 'a>;

    fn add(
        &mut self,
        group: u8,
        frequency: DelayUnit,
        rate: ZeroToOneF32,
    ) -> Result<(), AddDelayError>;

    fn remove(&mut self, index: usize) -> bool;
}

pub trait HasDelays {
    type Delays: Delays + Sized;
    type DelaysMut: DelaysMut + Sized;

    fn delays(&self) -> BoxOrRef<Self::Delays>;
    fn delays_mut(&mut self) -> BoxOrMut<Self::DelaysMut>;
}
