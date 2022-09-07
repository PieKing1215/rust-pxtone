//! Basic `GenericEventKind` and `GenericEvent` implementations

use std::borrow::Borrow;

use crate::pxtone::util::{BoxOrMut, BoxOrRef};

use super::event::{
    EventGroupNo, EventKey, EventOn, EventPanTime, EventPanVolume, EventPorta, EventTuning,
    EventVelocity, EventVoiceNo, EventVolume, GenericEvent, GenericEventKind, GenericEventKindMut,
    GenericEventKindRef, PanValue, TuningValue, ZeroToOneF32,
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

pub struct EventImpl<'a> {
    pub clock: u32,
    pub unit_no: u8,
    pub kind: EventKindImpl<'a>,
}

impl EventOn for u32 {
    fn length(&self) -> u32 {
        *self
    }

    fn set_length(&mut self, length: u32) {
        *self = length;
    }
}

impl EventKey for i32 {
    fn key(&self) -> i32 {
        *self
    }

    fn set_key(&mut self, key: i32) {
        *self = key;
    }
}

impl EventPanVolume for PanValue {
    fn pan_volume(&self) -> PanValue {
        *self
    }

    fn set_pan_volume(&mut self, pan_volume: PanValue) {
        *self = pan_volume;
    }
}

impl EventVelocity for ZeroToOneF32 {
    fn velocity(&self) -> ZeroToOneF32 {
        *self
    }

    fn set_velocity(&mut self, velocity: ZeroToOneF32) {
        *self = velocity;
    }
}

impl EventVolume for ZeroToOneF32 {
    fn volume(&self) -> ZeroToOneF32 {
        *self
    }

    fn set_volume(&mut self, volume: ZeroToOneF32) {
        *self = volume;
    }
}

impl EventPorta for u32 {
    fn porta(&self) -> u32 {
        *self
    }

    fn set_porta(&mut self, porta: u32) {
        *self = porta;
    }
}

impl EventVoiceNo for u8 {
    fn voice_no(&self) -> u8 {
        *self
    }

    fn set_voice_no(&mut self, voice_no: u8) {
        *self = voice_no;
    }
}

impl EventGroupNo for u8 {
    fn group_no(&self) -> u8 {
        *self
    }

    fn set_group_no(&mut self, group_no: u8) {
        *self = group_no;
    }
}

impl EventTuning for TuningValue {
    fn tuning(&self) -> TuningValue {
        *self
    }

    fn set_tuning(&mut self, tuning: TuningValue) {
        *self = tuning;
    }
}

impl EventPanTime for PanValue {
    fn pan_time(&self) -> PanValue {
        *self
    }

    fn set_pan_time(&mut self, pan_time: PanValue) {
        *self = pan_time;
    }
}

impl GenericEvent for EventImpl<'_> {
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

    fn kind(&self) -> GenericEventKindRef {
        match &self.kind {
            GenericEventKind::Invalid => todo!(),
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
            GenericEventKind::Invalid => todo!(),
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
