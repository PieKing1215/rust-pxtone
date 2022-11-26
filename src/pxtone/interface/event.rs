use std::{
    borrow::Borrow,
    convert::Infallible,
    fmt::{self, Debug},
    marker::PhantomData,
    ops::Deref,
};

use crate::pxtone::util::{BoxOrMut, BoxOrRef, ZeroToOneF32};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub trait BaseEvent {
    fn clock(&self) -> u32;
    fn set_clock(&mut self, clock: u32);

    fn unit_no(&self) -> u8;
    fn set_unit_no(&mut self, unit_no: u8);
}

pub trait GenericEvent: BaseEvent {
    type On: EventOn + ?Sized;
    type Key: EventKey + ?Sized;
    type PanVolume: EventPanVolume + ?Sized;
    type Velocity: EventVelocity + ?Sized;
    type Volume: EventVolume + ?Sized;
    type Porta: EventPorta + ?Sized;
    type VoiceNo: EventVoiceNo + ?Sized;
    type GroupNo: EventGroupNo + ?Sized;
    type Tuning: EventTuning + ?Sized;
    type PanTime: EventPanTime + ?Sized;

    #[allow(clippy::type_complexity)] // I can't think of a good way to make this less complex
    fn kind(
        &self,
    ) -> GenericEventKindRef<
        Self::On,
        Self::Key,
        Self::PanVolume,
        Self::Velocity,
        Self::Volume,
        Self::Porta,
        Self::VoiceNo,
        Self::GroupNo,
        Self::Tuning,
        Self::PanTime,
    >;
    #[allow(clippy::type_complexity)] // I can't think of a good way to make this less complex
    fn kind_mut(
        &mut self,
    ) -> GenericEventKindMut<
        Self::On,
        Self::Key,
        Self::PanVolume,
        Self::Velocity,
        Self::Volume,
        Self::Porta,
        Self::VoiceNo,
        Self::GroupNo,
        Self::Tuning,
        Self::PanTime,
    >;
}

pub enum GenericEventKind<
    'a,
    On: EventOn + ?Sized,
    Key: EventKey + ?Sized,
    PanVolume: EventPanVolume + ?Sized,
    Velocity: EventVelocity + ?Sized,
    Volume: EventVolume + ?Sized,
    Porta: EventPorta + ?Sized,
    VoiceNo: EventVoiceNo + ?Sized,
    GroupNo: EventGroupNo + ?Sized,
    Tuning: EventTuning + ?Sized,
    PanTime: EventPanTime + ?Sized,
    BOn: Borrow<On>,
    BKey: Borrow<Key>,
    BPanVolume: Borrow<PanVolume>,
    BVelocity: Borrow<Velocity>,
    BVolume: Borrow<Volume>,
    BPorta: Borrow<Porta>,
    BVoiceNo: Borrow<VoiceNo>,
    BGroupNo: Borrow<GroupNo>,
    BTuning: Borrow<Tuning>,
    BPanTime: Borrow<PanTime>,
> {
    Invalid,
    On(BOn),
    Key(BKey),
    PanVolume(BPanVolume),
    Velocity(BVelocity),
    Volume(BVolume),
    Porta(BPorta),
    VoiceNo(BVoiceNo),
    GroupNo(BGroupNo),
    Tuning(BTuning),
    PanTime(BPanTime),

    // TODO: consider other ways to do this
    // relevant: https://github.com/rust-lang/rust/issues/32739
    /// Implementation detail needed to manipulate lifetimes/generics.
    #[allow(clippy::type_complexity)]
    _Phantom(
        Infallible,
        PhantomData<&'a (
            &'a On,
            &'a Key,
            &'a PanVolume,
            &'a Velocity,
            &'a Volume,
            &'a Porta,
            &'a VoiceNo,
            &'a GroupNo,
            &'a Tuning,
            &'a PanTime,
        )>,
    ),
}

// impl Debug for GenericEventKind
impl<
        'a,
        On: EventOn + ?Sized,
        Key: EventKey + ?Sized,
        PanVolume: EventPanVolume + ?Sized,
        Velocity: EventVelocity + ?Sized,
        Volume: EventVolume + ?Sized,
        Porta: EventPorta + ?Sized,
        VoiceNo: EventVoiceNo + ?Sized,
        GroupNo: EventGroupNo + ?Sized,
        Tuning: EventTuning + ?Sized,
        PanTime: EventPanTime + ?Sized,
        BOn: Borrow<On>,
        BKey: Borrow<Key>,
        BPanVolume: Borrow<PanVolume>,
        BVelocity: Borrow<Velocity>,
        BVolume: Borrow<Volume>,
        BPorta: Borrow<Porta>,
        BVoiceNo: Borrow<VoiceNo>,
        BGroupNo: Borrow<GroupNo>,
        BTuning: Borrow<Tuning>,
        BPanTime: Borrow<PanTime>,
    > Debug
    for GenericEventKind<
        'a,
        On,
        Key,
        PanVolume,
        Velocity,
        Volume,
        Porta,
        VoiceNo,
        GroupNo,
        Tuning,
        PanTime,
        BOn,
        BKey,
        BPanVolume,
        BVelocity,
        BVolume,
        BPorta,
        BVoiceNo,
        BGroupNo,
        BTuning,
        BPanTime,
    >
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid"),
            Self::On(e) => f.debug_tuple("On").field(&e.borrow().length()).finish(),
            Self::Key(e) => f.debug_tuple("Key").field(&e.borrow().key()).finish(),
            Self::PanVolume(e) => f
                .debug_tuple("PanVolume")
                .field(&e.borrow().pan_volume())
                .finish(),
            Self::Velocity(e) => f
                .debug_tuple("Velocity")
                .field(&e.borrow().velocity())
                .finish(),
            Self::Volume(e) => f.debug_tuple("Volume").field(&e.borrow().volume()).finish(),
            Self::Porta(e) => f.debug_tuple("Porta").field(&e.borrow().porta()).finish(),
            Self::VoiceNo(e) => f
                .debug_tuple("VoiceNo")
                .field(&e.borrow().voice_no())
                .finish(),
            Self::GroupNo(e) => f
                .debug_tuple("GroupNo")
                .field(&e.borrow().group_no())
                .finish(),
            Self::Tuning(e) => f.debug_tuple("Tuning").field(&e.borrow().tuning()).finish(),
            Self::PanTime(e) => f
                .debug_tuple("PanTime")
                .field(&e.borrow().pan_time())
                .finish(),
            Self::_Phantom(arg0, arg1) => {
                f.debug_tuple("_Phantom").field(arg0).field(arg1).finish()
            },
        }
    }
}

pub trait EventOn: BaseEvent {
    fn length(&self) -> u32;
    fn set_length(&mut self, length: u32);
}

pub trait EventKey: BaseEvent {
    fn key(&self) -> i32;
    fn set_key(&mut self, key: i32);
}

/// Wrapper for an f32 representing a pan value.
///
/// 0.0 means centered, -1.0 means full left, and 1.0 means full right.
#[derive(Clone, Copy, Debug)]
pub struct PanValue(f32);

impl PanValue {
    /// Create a `PanValue` from a normal f32.
    ///
    /// The value is clamped if outside -1.0..=1.0.
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(-1.0, 1.0))
    }

    #[must_use]
    pub const fn left() -> Self {
        Self(-1.0)
    }

    #[must_use]
    pub const fn center() -> Self {
        Self(0.0)
    }

    #[must_use]
    pub const fn right() -> Self {
        Self(1.0)
    }
}

impl Default for PanValue {
    fn default() -> Self {
        Self::center()
    }
}

impl Deref for PanValue {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait EventPanVolume: BaseEvent {
    fn pan_volume(&self) -> PanValue;
    fn set_pan_volume(&mut self, pan_volume: PanValue);
}

pub trait EventVelocity: BaseEvent {
    fn velocity(&self) -> ZeroToOneF32;
    fn set_velocity(&mut self, velocity: ZeroToOneF32);
}

pub trait EventVolume: BaseEvent {
    fn volume(&self) -> ZeroToOneF32;
    fn set_volume(&mut self, volume: ZeroToOneF32);
}

pub trait EventPorta: BaseEvent {
    fn porta(&self) -> u32;
    fn set_porta(&mut self, porta: u32);
}

pub trait EventVoiceNo: BaseEvent {
    fn voice_no(&self) -> u8;
    fn set_voice_no(&mut self, voice_no: u8);
}

pub trait EventGroupNo: BaseEvent {
    fn group_no(&self) -> u8;
    fn set_group_no(&mut self, group_no: u8);
}

/// Wrapper for an f32 representing a value from 0.0 to 9.99999 (inclusive).
#[derive(Clone, Copy, Debug)]
pub struct TuningValue(f32);

impl TuningValue {
    /// Create a `TuningValue` from a normal f32.
    ///
    /// The value is clamped if outside 0.0..=9.99999.
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 9.99999))
    }
}

impl Default for TuningValue {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Deref for TuningValue {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait EventTuning: BaseEvent {
    fn tuning(&self) -> TuningValue;
    fn set_tuning(&mut self, tuning: TuningValue);
}

pub trait EventPanTime: BaseEvent {
    fn pan_time(&self) -> PanValue;
    fn set_pan_time(&mut self, pan_time: PanValue);
}

pub type GenericEventKindRef<
    'a,
    On,
    Key,
    PanVolume,
    Velocity,
    Volume,
    Porta,
    VoiceNo,
    GroupNo,
    Tuning,
    PanTime,
> = GenericEventKind<
    'a,
    On,
    Key,
    PanVolume,
    Velocity,
    Volume,
    Porta,
    VoiceNo,
    GroupNo,
    Tuning,
    PanTime,
    BoxOrRef<'a, On>,
    BoxOrRef<'a, Key>,
    BoxOrRef<'a, PanVolume>,
    BoxOrRef<'a, Velocity>,
    BoxOrRef<'a, Volume>,
    BoxOrRef<'a, Porta>,
    BoxOrRef<'a, VoiceNo>,
    BoxOrRef<'a, GroupNo>,
    BoxOrRef<'a, Tuning>,
    BoxOrRef<'a, PanTime>,
>;

pub type GenericEventKindMut<
    'a,
    On,
    Key,
    PanVolume,
    Velocity,
    Volume,
    Porta,
    VoiceNo,
    GroupNo,
    Tuning,
    PanTime,
> = GenericEventKind<
    'a,
    On,
    Key,
    PanVolume,
    Velocity,
    Volume,
    Porta,
    VoiceNo,
    GroupNo,
    Tuning,
    PanTime,
    BoxOrMut<'a, On>,
    BoxOrMut<'a, Key>,
    BoxOrMut<'a, PanVolume>,
    BoxOrMut<'a, Velocity>,
    BoxOrMut<'a, Volume>,
    BoxOrMut<'a, Porta>,
    BoxOrMut<'a, VoiceNo>,
    BoxOrMut<'a, GroupNo>,
    BoxOrMut<'a, Tuning>,
    BoxOrMut<'a, PanTime>,
>;

pub trait EventList {
    type Event: GenericEvent;

    fn iter(&self) -> Box<dyn Iterator<Item = &Self::Event> + '_>;
}

#[derive(Debug)]
pub struct AddEventError {}

impl fmt::Display for AddEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to add Event")
    }
}

impl std::error::Error for AddEventError {}

pub trait EventListMut: EventList {
    fn iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Event> + '_>;

    fn add<E: GenericEvent>(&mut self, event: &E) -> Result<(), AddEventError>;
}

pub trait HasEventList {
    type EventList<'a>: EventList + Sized
    where
        Self: 'a;
    type EventListMut<'a>: EventListMut + Sized
    where
        Self: 'a;

    fn event_list(&self) -> BoxOrRef<Self::EventList<'_>>;
    fn event_list_mut(&mut self) -> BoxOrMut<Self::EventListMut<'_>>;
}
