use std::{
    borrow::Borrow,
    convert::Infallible,
    fmt::{self, Debug},
    marker::PhantomData,
    ops::Deref,
};

use crate::pxtone::util::{BoxOrMut, BoxOrRef};

pub trait GenericEvent {
    fn clock(&self) -> u32;
    fn set_clock(&mut self, clock: u32);

    fn unit_no(&self) -> u8;
    fn set_unit_no(&mut self, unit_no: u8);

    fn kind(&self) -> GenericEventKindRef;
    fn kind_mut(&mut self) -> GenericEventKindMut;
}

pub enum GenericEventKind<
    'a,
    On: Borrow<dyn EventOn + 'a>,
    Key: Borrow<dyn EventKey + 'a>,
    PanVolume: Borrow<dyn EventPanVolume + 'a>,
    Velocity: Borrow<dyn EventVelocity + 'a>,
    Volume: Borrow<dyn EventVolume + 'a>,
    Porta: Borrow<dyn EventPorta + 'a>,
    VoiceNo: Borrow<dyn EventVoiceNo + 'a>,
    GroupNo: Borrow<dyn EventGroupNo + 'a>,
    Tuning: Borrow<dyn EventTuning + 'a>,
    PanTime: Borrow<dyn EventPanTime + 'a>,
> {
    Invalid,
    On(On),
    Key(Key),
    PanVolume(PanVolume),
    Velocity(Velocity),
    Volume(Volume),
    Porta(Porta),
    VoiceNo(VoiceNo),
    GroupNo(GroupNo),
    Tuning(Tuning),
    PanTime(PanTime),

    // TODO: consider other ways to do this
    // relevant: https://github.com/rust-lang/rust/issues/32739
    /// Implementation detail needed to hold a lifetime.
    _Phantom(Infallible, PhantomData<&'a ()>),
}

// impl Debug for GenericEventKind
impl<
        'a,
        On: Borrow<dyn EventOn + 'a>,
        Key: Borrow<dyn EventKey + 'a>,
        PanVolume: Borrow<dyn EventPanVolume + 'a>,
        Velocity: Borrow<dyn EventVelocity + 'a>,
        Volume: Borrow<dyn EventVolume + 'a>,
        Porta: Borrow<dyn EventPorta + 'a>,
        VoiceNo: Borrow<dyn EventVoiceNo + 'a>,
        GroupNo: Borrow<dyn EventGroupNo + 'a>,
        Tuning: Borrow<dyn EventTuning + 'a>,
        PanTime: Borrow<dyn EventPanTime + 'a>,
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

pub trait EventOn {
    fn length(&self) -> u32;
    fn set_length(&mut self, length: u32);
}

pub trait EventKey {
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

pub trait EventPanVolume {
    fn pan_volume(&self) -> PanValue;
    fn set_pan_volume(&mut self, pan_volume: PanValue);
}

/// Wrapper for an f32 representing a value from 0.0 to 1.0 (inclusive).
#[derive(Clone, Copy, Debug)]
pub struct ZeroToOneF32(f32);

impl ZeroToOneF32 {
    /// Create a `ZeroToOneF32` from a normal f32.
    ///
    /// The value is clamped if outside 0.0..=1.0.
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }
}

impl Default for ZeroToOneF32 {
    fn default() -> Self {
        Self(104.0 / 128.0)
    }
}

impl Deref for ZeroToOneF32 {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait EventVelocity {
    fn velocity(&self) -> ZeroToOneF32;
    fn set_velocity(&mut self, velocity: ZeroToOneF32);
}

pub trait EventVolume {
    fn volume(&self) -> ZeroToOneF32;
    fn set_volume(&mut self, volume: ZeroToOneF32);
}

pub trait EventPorta {
    fn porta(&self) -> u32;
    fn set_porta(&mut self, porta: u32);
}

pub trait EventVoiceNo {
    fn voice_no(&self) -> u8;
    fn set_voice_no(&mut self, voice_no: u8);
}

pub trait EventGroupNo {
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

pub trait EventTuning {
    fn tuning(&self) -> TuningValue;
    fn set_tuning(&mut self, tuning: TuningValue);
}

pub trait EventPanTime {
    fn pan_time(&self) -> PanValue;
    fn set_pan_time(&mut self, pan_time: PanValue);
}

pub type GenericEventKindRef<'a> = GenericEventKind<
    'a,
    BoxOrRef<'a, dyn EventOn>,
    BoxOrRef<'a, dyn EventKey>,
    BoxOrRef<'a, dyn EventPanVolume>,
    BoxOrRef<'a, dyn EventVelocity>,
    BoxOrRef<'a, dyn EventVolume>,
    BoxOrRef<'a, dyn EventPorta>,
    BoxOrRef<'a, dyn EventVoiceNo>,
    BoxOrRef<'a, dyn EventGroupNo>,
    BoxOrRef<'a, dyn EventTuning>,
    BoxOrRef<'a, dyn EventPanTime>,
>;

pub type GenericEventKindMut<'a> = GenericEventKind<
    'a,
    BoxOrMut<'a, dyn EventOn>,
    BoxOrMut<'a, dyn EventKey>,
    BoxOrMut<'a, dyn EventPanVolume>,
    BoxOrMut<'a, dyn EventVelocity>,
    BoxOrMut<'a, dyn EventVolume>,
    BoxOrMut<'a, dyn EventPorta>,
    BoxOrMut<'a, dyn EventVoiceNo>,
    BoxOrMut<'a, dyn EventGroupNo>,
    BoxOrMut<'a, dyn EventTuning>,
    BoxOrMut<'a, dyn EventPanTime>,
>;

pub trait EventList {
    fn iter(&self) -> Box<dyn Iterator<Item = &dyn GenericEvent>>;
}

#[derive(Debug)]
pub struct AddEventError {}

impl fmt::Display for AddEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Text")
    }
}

impl std::error::Error for AddEventError {}

pub trait EventListMut: EventList {
    fn iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut dyn GenericEvent>>;

    // TODO: make this an enum or something so you can't input invalid data
    fn add(&mut self, event: &dyn GenericEvent) -> Result<(), AddEventError>;
}
