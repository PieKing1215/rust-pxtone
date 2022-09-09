use std::fmt;

use crate::util::{BoxOrMut, BoxOrRef};

use super::{
    delay::{Delays, DelaysMut},
    event::{EventList, EventListMut},
    overdrive::{OverDrives, OverDrivesMut},
    unit::{Units, UnitsMut},
    woice::{Woices, WoicesMut},
};

#[derive(Debug)]
pub struct InvalidText;

impl fmt::Display for InvalidText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Text")
    }
}

impl std::error::Error for InvalidText {}

/// Base trait for the overall state of pxtone
pub trait PxTone {
    type Units: Units + Sized;
    type UnitsMut: UnitsMut + Sized;
    type EventList: EventList + Sized;
    type EventListMut: EventListMut + Sized;
    type Woices: Woices + Sized;
    type WoicesMut: WoicesMut + Sized;
    type Delays: Delays + Sized;
    type DelaysMut: DelaysMut + Sized;
    type OverDrives: OverDrives + Sized;
    type OverDrivesMut: OverDrivesMut + Sized;

    fn beat_num(&self) -> i32;
    fn set_beat_num(&mut self, beat_num: i32);

    fn beat_tempo(&self) -> f32;
    fn set_beat_tempo(&mut self, beat_tempo: f32);

    fn beat_clock(&self) -> i32;
    fn set_beat_clock(&mut self, beat_clock: i32);

    fn num_measures(&self) -> i32;
    fn set_num_measures(&mut self, num_measures: i32);

    fn repeat_measure(&self) -> i32;
    fn set_repeat_measure(&mut self, repeat_measure: i32);

    fn last_measure(&self) -> i32;
    fn set_last_measure(&mut self, last_measure: i32);

    fn name(&self) -> String;
    fn set_name(&mut self, name: String) -> Result<(), InvalidText>;

    fn comment(&self) -> String;
    fn set_comment(&mut self, comment: String) -> Result<(), InvalidText>;

    fn units(&self) -> BoxOrRef<Self::Units>;
    fn units_mut(&mut self) -> BoxOrMut<Self::UnitsMut>;

    fn event_list(&self) -> BoxOrRef<Self::EventList>;
    fn event_list_mut(&mut self) -> BoxOrMut<Self::EventListMut>;

    fn woices(&self) -> BoxOrRef<Self::Woices>;
    fn woices_mut(&mut self) -> BoxOrMut<Self::WoicesMut>;

    fn delays(&self) -> BoxOrRef<Self::Delays>;
    fn delays_mut(&mut self) -> BoxOrMut<Self::DelaysMut>;

    fn overdrives(&self) -> BoxOrRef<Self::OverDrives>;
    fn overdrives_mut(&mut self) -> BoxOrMut<Self::OverDrivesMut>;
}
