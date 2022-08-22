
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum EventKind {
    Null,
    On,
    Key,
    PanVolume,
    Velocity,
    Volume,
    Portament,
    BeatClock,
    BeatTempo,
    BeatNum,
    Repeat,
    Last,
    VoiceNo,
    GroupNo,
    Tuning,
    PanTime,
}

impl From<u8> for EventKind {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::On,
            2 => Self::Key,
            3 => Self::PanVolume,
            4 => Self::Velocity,
            5 => Self::Volume,
            6 => Self::Portament,
            7 => Self::BeatClock,
            8 => Self::BeatTempo,
            9 => Self::BeatNum,
            10 => Self::Repeat,
            11 => Self::Last,
            12 => Self::VoiceNo,
            13 => Self::GroupNo,
            14 => Self::Tuning,
            15 => Self::PanTime,
            _ => Self::Null,
        }
    }
}

pub trait Event {
    fn kind(&self) -> EventKind;

    fn unit_no(&self) -> u8;
    fn set_unit_no(&mut self, u: u8);

    fn value(&self) -> i32;
    fn set_value(&mut self, v: i32);

    fn clock(&self) -> i32;
    fn set_clock(&mut self, c: i32);
}

pub trait EventList {
    type E: Event + 'static;
    type I: IntoIterator<Item = &'static Self::E>;

    fn events(&self) -> Self::I;
}

pub trait EventListMut {
    type E: Event + 'static;
    type I: IntoIterator<Item = &'static Self::E>;
    type IM: IntoIterator<Item = &'static mut Self::E>;

    fn events(&self) -> Self::I;
    fn events_mut(&mut self) -> Self::IM;
}