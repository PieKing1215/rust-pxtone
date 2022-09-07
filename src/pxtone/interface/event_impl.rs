//! Basic `GenericEventKind` and `GenericEvent` implementations

use std::borrow::Borrow;

use crate::pxtone::util::{BoxOrMut, BoxOrRef};

use super::event::{
    BaseEvent, EventGroupNo, EventKey, EventOn, EventPanTime, EventPanVolume, EventPorta,
    EventTuning, EventVelocity, EventVoiceNo, EventVolume, GenericEvent, GenericEventKind,
    GenericEventKindMut, GenericEventKindRef, PanValue, TuningValue, ZeroToOneF32,
};

pub type EventKindImpl<'a> = GenericEventKind<
    'a,
    Box<dyn EventOn + 'a>,
    Box<dyn EventKey + 'a>,
    Box<dyn EventPanVolume + 'a>,
    Box<dyn EventVelocity + 'a>,
    Box<dyn EventVolume + 'a>,
    Box<dyn EventPorta + 'a>,
    Box<dyn EventVoiceNo + 'a>,
    Box<dyn EventGroupNo + 'a>,
    Box<dyn EventTuning + 'a>,
    Box<dyn EventPanTime + 'a>,
>;

pub struct BaseEventImpl {
    pub clock: u32,
    pub unit_no: u8,
}

impl BaseEvent for BaseEventImpl {
    fn clock(&self) -> u32 {
        self.clock
    }

    fn set_clock(&mut self, clock: u32) {
        self.clock = clock;
    }

    fn unit_no(&self) -> u8 {
        self.unit_no
    }

    fn set_unit_no(&mut self, unit_no: u8) {
        self.unit_no = unit_no;
    }
}

pub struct EventImpl<'a> {
    pub kind: EventKindImpl<'a>,
}

impl EventImpl<'_> {
    #[must_use]
    pub fn on(clock: u32, unit_no: u8, length: u32) -> Self {
        Self {
            kind: EventKindImpl::On(Box::new((BaseEventImpl { clock, unit_no }, length))),
        }
    }

    #[must_use]
    pub fn key(clock: u32, unit_no: u8, key: i32) -> Self {
        Self {
            kind: EventKindImpl::Key(Box::new((BaseEventImpl { clock, unit_no }, key))),
        }
    }

    #[must_use]
    pub fn pan_volume(clock: u32, unit_no: u8, pan_volume: PanValue) -> Self {
        Self {
            kind: EventKindImpl::PanVolume(Box::new((
                BaseEventImpl { clock, unit_no },
                pan_volume,
            ))),
        }
    }

    #[must_use]
    pub fn velocity(clock: u32, unit_no: u8, velocity: ZeroToOneF32) -> Self {
        Self {
            kind: EventKindImpl::Velocity(Box::new((BaseEventImpl { clock, unit_no }, velocity))),
        }
    }

    #[must_use]
    pub fn volume(clock: u32, unit_no: u8, volume: ZeroToOneF32) -> Self {
        Self {
            kind: EventKindImpl::Volume(Box::new((BaseEventImpl { clock, unit_no }, volume))),
        }
    }

    #[must_use]
    pub fn porta(clock: u32, unit_no: u8, porta: u32) -> Self {
        Self {
            kind: EventKindImpl::Porta(Box::new((BaseEventImpl { clock, unit_no }, porta))),
        }
    }

    #[must_use]
    pub fn voice_no(clock: u32, unit_no: u8, voice_no: u8) -> Self {
        Self {
            kind: EventKindImpl::VoiceNo(Box::new((BaseEventImpl { clock, unit_no }, voice_no))),
        }
    }

    #[must_use]
    pub fn group_no(clock: u32, unit_no: u8, group_no: u8) -> Self {
        Self {
            kind: EventKindImpl::VoiceNo(Box::new((BaseEventImpl { clock, unit_no }, group_no))),
        }
    }

    #[must_use]
    pub fn tuning(clock: u32, unit_no: u8, tuning: TuningValue) -> Self {
        Self {
            kind: EventKindImpl::Tuning(Box::new((BaseEventImpl { clock, unit_no }, tuning))),
        }
    }

    #[must_use]
    pub fn pan_time(clock: u32, unit_no: u8, pan_time: PanValue) -> Self {
        Self {
            kind: EventKindImpl::PanTime(Box::new((BaseEventImpl { clock, unit_no }, pan_time))),
        }
    }
}

impl<T> BaseEvent for (BaseEventImpl, T) {
    fn clock(&self) -> u32 {
        self.0.clock()
    }

    fn set_clock(&mut self, clock: u32) {
        self.0.set_clock(clock);
    }

    fn unit_no(&self) -> u8 {
        self.0.unit_no()
    }

    fn set_unit_no(&mut self, unit_no: u8) {
        self.0.set_unit_no(unit_no);
    }
}

impl EventOn for (BaseEventImpl, u32) {
    fn length(&self) -> u32 {
        self.1
    }

    fn set_length(&mut self, length: u32) {
        self.1 = length;
    }
}

impl EventKey for (BaseEventImpl, i32) {
    fn key(&self) -> i32 {
        self.1
    }

    fn set_key(&mut self, key: i32) {
        self.1 = key;
    }
}

impl EventPanVolume for (BaseEventImpl, PanValue) {
    fn pan_volume(&self) -> PanValue {
        self.1
    }

    fn set_pan_volume(&mut self, pan_volume: PanValue) {
        self.1 = pan_volume;
    }
}

impl EventVelocity for (BaseEventImpl, ZeroToOneF32) {
    fn velocity(&self) -> ZeroToOneF32 {
        self.1
    }

    fn set_velocity(&mut self, velocity: ZeroToOneF32) {
        self.1 = velocity;
    }
}

impl EventVolume for (BaseEventImpl, ZeroToOneF32) {
    fn volume(&self) -> ZeroToOneF32 {
        self.1
    }

    fn set_volume(&mut self, volume: ZeroToOneF32) {
        self.1 = volume;
    }
}

impl EventPorta for (BaseEventImpl, u32) {
    fn porta(&self) -> u32 {
        self.1
    }

    fn set_porta(&mut self, porta: u32) {
        self.1 = porta;
    }
}

impl EventVoiceNo for (BaseEventImpl, u8) {
    fn voice_no(&self) -> u8 {
        self.1
    }

    fn set_voice_no(&mut self, voice_no: u8) {
        self.1 = voice_no;
    }
}

impl EventGroupNo for (BaseEventImpl, u8) {
    fn group_no(&self) -> u8 {
        self.1
    }

    fn set_group_no(&mut self, group_no: u8) {
        self.1 = group_no;
    }
}

impl EventTuning for (BaseEventImpl, TuningValue) {
    fn tuning(&self) -> TuningValue {
        self.1
    }

    fn set_tuning(&mut self, tuning: TuningValue) {
        self.1 = tuning;
    }
}

impl EventPanTime for (BaseEventImpl, PanValue) {
    fn pan_time(&self) -> PanValue {
        self.1
    }

    fn set_pan_time(&mut self, pan_time: PanValue) {
        self.1 = pan_time;
    }
}

impl BaseEvent for EventImpl<'_> {
    fn clock(&self) -> u32 {
        match &self.kind {
            GenericEventKind::On(e) => e.clock(),
            GenericEventKind::Key(e) => e.clock(),
            GenericEventKind::PanVolume(e) => e.clock(),
            GenericEventKind::Velocity(e) => e.clock(),
            GenericEventKind::Volume(e) => e.clock(),
            GenericEventKind::Porta(e) => e.clock(),
            GenericEventKind::VoiceNo(e) => e.clock(),
            GenericEventKind::GroupNo(e) => e.clock(),
            GenericEventKind::Tuning(e) => e.clock(),
            GenericEventKind::PanTime(e) => e.clock(),
            _ => 0,
        }
    }

    fn set_clock(&mut self, clock: u32) {
        match &mut self.kind {
            GenericEventKind::On(e) => e.set_clock(clock),
            GenericEventKind::Key(e) => e.set_clock(clock),
            GenericEventKind::PanVolume(e) => e.set_clock(clock),
            GenericEventKind::Velocity(e) => e.set_clock(clock),
            GenericEventKind::Volume(e) => e.set_clock(clock),
            GenericEventKind::Porta(e) => e.set_clock(clock),
            GenericEventKind::VoiceNo(e) => e.set_clock(clock),
            GenericEventKind::GroupNo(e) => e.set_clock(clock),
            GenericEventKind::Tuning(e) => e.set_clock(clock),
            GenericEventKind::PanTime(e) => e.set_clock(clock),
            _ => {},
        };
    }

    fn unit_no(&self) -> u8 {
        match &self.kind {
            GenericEventKind::On(e) => e.unit_no(),
            GenericEventKind::Key(e) => e.unit_no(),
            GenericEventKind::PanVolume(e) => e.unit_no(),
            GenericEventKind::Velocity(e) => e.unit_no(),
            GenericEventKind::Volume(e) => e.unit_no(),
            GenericEventKind::Porta(e) => e.unit_no(),
            GenericEventKind::VoiceNo(e) => e.unit_no(),
            GenericEventKind::GroupNo(e) => e.unit_no(),
            GenericEventKind::Tuning(e) => e.unit_no(),
            GenericEventKind::PanTime(e) => e.unit_no(),
            _ => 0,
        }
    }

    fn set_unit_no(&mut self, unit_no: u8) {
        match &mut self.kind {
            GenericEventKind::On(e) => e.set_unit_no(unit_no),
            GenericEventKind::Key(e) => e.set_unit_no(unit_no),
            GenericEventKind::PanVolume(e) => e.set_unit_no(unit_no),
            GenericEventKind::Velocity(e) => e.set_unit_no(unit_no),
            GenericEventKind::Volume(e) => e.set_unit_no(unit_no),
            GenericEventKind::Porta(e) => e.set_unit_no(unit_no),
            GenericEventKind::VoiceNo(e) => e.set_unit_no(unit_no),
            GenericEventKind::GroupNo(e) => e.set_unit_no(unit_no),
            GenericEventKind::Tuning(e) => e.set_unit_no(unit_no),
            GenericEventKind::PanTime(e) => e.set_unit_no(unit_no),
            _ => {},
        };
    }
}

impl GenericEvent for EventImpl<'_> {
    fn kind(&self) -> GenericEventKindRef {
        match &self.kind {
            GenericEventKind::Invalid => GenericEventKind::Invalid,
            GenericEventKind::On(e) => GenericEventKind::On(BoxOrRef::Ref(e.borrow())),
            GenericEventKind::Key(e) => GenericEventKind::Key(BoxOrRef::Ref(e.borrow())),
            GenericEventKind::PanVolume(e) => {
                GenericEventKind::PanVolume(BoxOrRef::Ref(e.borrow()))
            },
            GenericEventKind::Velocity(e) => GenericEventKind::Velocity(BoxOrRef::Ref(e.borrow())),
            GenericEventKind::Volume(e) => GenericEventKind::Volume(BoxOrRef::Ref(e.borrow())),
            GenericEventKind::Porta(e) => GenericEventKind::Porta(BoxOrRef::Ref(e.borrow())),
            GenericEventKind::VoiceNo(e) => GenericEventKind::VoiceNo(BoxOrRef::Ref(e.borrow())),
            GenericEventKind::GroupNo(e) => GenericEventKind::GroupNo(BoxOrRef::Ref(e.borrow())),
            GenericEventKind::Tuning(e) => GenericEventKind::Tuning(BoxOrRef::Ref(e.borrow())),
            GenericEventKind::PanTime(e) => GenericEventKind::PanTime(BoxOrRef::Ref(e.borrow())),
            GenericEventKind::_Phantom(a, b) => GenericEventKind::_Phantom(*a, *b),
        }
    }

    fn kind_mut(&mut self) -> GenericEventKindMut {
        match &mut self.kind {
            GenericEventKind::Invalid => GenericEventKind::Invalid,
            GenericEventKind::On(e) => GenericEventKind::On(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::Key(e) => GenericEventKind::Key(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::PanVolume(e) => GenericEventKind::PanVolume(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::Velocity(e) => GenericEventKind::Velocity(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::Volume(e) => GenericEventKind::Volume(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::Porta(e) => GenericEventKind::Porta(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::VoiceNo(e) => GenericEventKind::VoiceNo(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::GroupNo(e) => GenericEventKind::GroupNo(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::Tuning(e) => GenericEventKind::Tuning(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::PanTime(e) => GenericEventKind::PanTime(BoxOrMut::Ref(&mut **e)),
            GenericEventKind::_Phantom(a, b) => GenericEventKind::_Phantom(*a, *b),
        }
    }
}
