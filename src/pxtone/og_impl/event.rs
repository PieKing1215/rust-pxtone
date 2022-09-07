use std::{
    borrow::{Borrow, BorrowMut},
    ptr::addr_of,
};

use pxtone_sys::{pxtnEvelist, EVERECORD};

use crate::{
    interface::event::{
        AddEventError, BaseEvent, EventGroupNo, EventKey, EventList, EventListMut, EventOn,
        EventPanTime, EventPanVolume, EventTuning, EventVelocity, EventVoiceNo, EventVolume,
        GenericEvent, GenericEventKind, GenericEventKindMut, GenericEventKindRef, PanValue,
        TuningValue, ZeroToOneF32,
    },
    pxtone::util::{BoxOrMut, BoxOrRef},
};

// PxToneEventList implementation

pub struct PxToneEventList<T: Borrow<pxtnEvelist>> {
    evelist: T,
}

impl<T: Borrow<pxtnEvelist>> PxToneEventList<T> {
    pub fn new(evelist: T) -> Self {
        Self { evelist }
    }
}

pub trait MaybeNext: Sized {
    type Map;
    fn next(&self) -> Option<Self>;
    fn map(&self) -> Self::Map;
}

impl MaybeNext for *const EVERECORD {
    type Map = &'static EVERECORD;

    fn next(&self) -> Option<Self> {
        if self.is_null() {
            None
        } else {
            Some(unsafe { **self }.next)
        }
    }

    fn map(&self) -> &'static EVERECORD {
        unsafe { &**self }
    }
}

impl MaybeNext for *mut EVERECORD {
    type Map = &'static mut EVERECORD;

    fn next(&self) -> Option<Self> {
        if self.is_null() {
            None
        } else {
            Some(unsafe { **self }.next)
        }
    }

    fn map(&self) -> &'static mut EVERECORD {
        unsafe { &mut **self }
    }
}

pub struct EventLinkedList<T: MaybeNext> {
    raw: T,
}

pub struct IterEventLinkedList<T: MaybeNext> {
    current: T,
}

impl<M, T: MaybeNext<Map = M>> Iterator for IterEventLinkedList<T> {
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.current.next() {
            let ret = self.current.map();
            self.current = next;

            Some(ret)
        } else {
            None
        }
    }
}

impl<M, T: MaybeNext<Map = M>> IntoIterator for EventLinkedList<T> {
    type Item = M;
    type IntoIter = IterEventLinkedList<T>;

    fn into_iter(self) -> Self::IntoIter {
        IterEventLinkedList { current: self.raw }
    }
}

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

impl BaseEvent for EVERECORD {
    fn clock(&self) -> u32 {
        self.clock as u32
    }

    fn set_clock(&mut self, clock: u32) {
        self.clock = clock as _;
    }

    fn unit_no(&self) -> u8 {
        self.unit_no
    }

    fn set_unit_no(&mut self, unit_no: u8) {
        self.unit_no = unit_no;
    }
}

impl GenericEvent for EVERECORD {
    fn kind(&self) -> GenericEventKindRef {
        match EventKind::from(self.kind) {
            EventKind::On => GenericEventKind::On(BoxOrRef::Ref(self)),
            EventKind::Key => GenericEventKind::Key(BoxOrRef::Ref(self)),
            EventKind::PanVolume => GenericEventKind::PanVolume(BoxOrRef::Ref(self)),
            EventKind::Velocity => GenericEventKind::Velocity(BoxOrRef::Ref(self)),
            EventKind::Volume => GenericEventKind::Volume(BoxOrRef::Ref(self)),
            EventKind::VoiceNo => GenericEventKind::VoiceNo(BoxOrRef::Ref(self)),
            EventKind::GroupNo => GenericEventKind::GroupNo(BoxOrRef::Ref(self)),
            EventKind::Tuning => GenericEventKind::Tuning(BoxOrRef::Ref(self)),
            EventKind::PanTime => GenericEventKind::PanTime(BoxOrRef::Ref(self)),
            _ => GenericEventKind::Invalid,
        }
    }

    fn kind_mut(&mut self) -> GenericEventKindMut {
        match EventKind::from(self.kind) {
            EventKind::On => GenericEventKind::On(BoxOrMut::Ref(self)),
            EventKind::Key => GenericEventKind::Key(BoxOrMut::Ref(self)),
            EventKind::PanVolume => GenericEventKind::PanVolume(BoxOrMut::Ref(self)),
            EventKind::Velocity => GenericEventKind::Velocity(BoxOrMut::Ref(self)),
            EventKind::Volume => GenericEventKind::Volume(BoxOrMut::Ref(self)),
            EventKind::VoiceNo => GenericEventKind::VoiceNo(BoxOrMut::Ref(self)),
            EventKind::GroupNo => GenericEventKind::GroupNo(BoxOrMut::Ref(self)),
            EventKind::Tuning => GenericEventKind::Tuning(BoxOrMut::Ref(self)),
            EventKind::PanTime => GenericEventKind::PanTime(BoxOrMut::Ref(self)),
            _ => GenericEventKind::Invalid,
        }
    }
}

impl EventOn for EVERECORD {
    fn length(&self) -> u32 {
        self.value as _
    }

    fn set_length(&mut self, length: u32) {
        self.value = length as _;
    }
}

impl EventKey for EVERECORD {
    fn key(&self) -> i32 {
        self.value
    }

    fn set_key(&mut self, key: i32) {
        self.value = key;
    }
}

impl EventPanVolume for EVERECORD {
    fn pan_volume(&self) -> PanValue {
        #[allow(clippy::cast_precision_loss)]
        PanValue::new((self.value as f32 / 128.0) * 2.0 - 1.0)
    }

    fn set_pan_volume(&mut self, pan_volume: PanValue) {
        self.value = ((*pan_volume / 2.0 + 0.5) * 128.0) as _;
    }
}

impl EventVelocity for EVERECORD {
    fn velocity(&self) -> ZeroToOneF32 {
        #[allow(clippy::cast_precision_loss)]
        ZeroToOneF32::new(self.value as f32 / 128.0)
    }

    fn set_velocity(&mut self, velocity: ZeroToOneF32) {
        self.value = (*velocity * 128.0) as _;
    }
}

impl EventVolume for EVERECORD {
    fn volume(&self) -> ZeroToOneF32 {
        #[allow(clippy::cast_precision_loss)]
        ZeroToOneF32::new(self.value as f32 / 128.0)
    }

    fn set_volume(&mut self, volume: ZeroToOneF32) {
        self.value = (*volume * 128.0) as _;
    }
}

impl EventVoiceNo for EVERECORD {
    fn voice_no(&self) -> u8 {
        self.value as _
    }

    fn set_voice_no(&mut self, voice_no: u8) {
        self.value = voice_no as _;
    }
}

impl EventGroupNo for EVERECORD {
    fn group_no(&self) -> u8 {
        self.value as _
    }

    fn set_group_no(&mut self, group_no: u8) {
        self.value = group_no as _;
    }
}

impl EventTuning for EVERECORD {
    fn tuning(&self) -> TuningValue {
        TuningValue::new(unsafe { *addr_of!(self.value).cast() })
    }

    fn set_tuning(&mut self, tuning: TuningValue) {
        self.value = unsafe { *addr_of!(*tuning).cast() };
    }
}

impl EventPanTime for EVERECORD {
    fn pan_time(&self) -> PanValue {
        #[allow(clippy::cast_precision_loss)]
        PanValue::new((self.value as f32 / 128.0) * 2.0 - 1.0)
    }

    fn set_pan_time(&mut self, pan_time: PanValue) {
        self.value = ((*pan_time / 2.0 + 0.5) * 128.0) as _;
    }
}

impl<T: BorrowMut<pxtnEvelist>> EventListMut for PxToneEventList<T> {
    fn iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut dyn GenericEvent>> {
        Box::new(
            EventLinkedList { raw: self.evelist.borrow_mut()._start }
                .into_iter()
                .map(|e: &'static mut EVERECORD| e as &mut dyn GenericEvent),
        )
    }

    fn add(&mut self, event: &dyn GenericEvent) -> Result<(), AddEventError> {
        unsafe {
            let (kind, value) = match event.kind() {
                GenericEventKind::On(e) => (EventKind::On, e.length() as _),
                GenericEventKind::Key(e) => (EventKind::Key, e.key()),
                GenericEventKind::PanVolume(e) => (
                    EventKind::PanVolume,
                    ((*e.pan_volume() / 2.0 + 0.5) * 128.0) as _,
                ),
                GenericEventKind::Velocity(e) => {
                    (EventKind::Velocity, (*e.velocity() * 128.0) as _)
                },
                GenericEventKind::Volume(e) => (EventKind::Volume, (*e.volume() * 128.0) as _),
                GenericEventKind::Porta(e) => (EventKind::Portament, e.porta() as _),
                GenericEventKind::VoiceNo(e) => (EventKind::VoiceNo, e.voice_no() as _),
                GenericEventKind::GroupNo(e) => (EventKind::GroupNo, e.group_no() as _),
                GenericEventKind::Tuning(e) => (EventKind::Tuning, *addr_of!(*e.tuning()).cast()),
                GenericEventKind::PanTime(e) => (
                    EventKind::PanTime,
                    ((*e.pan_time() / 2.0 + 0.5) * 128.0) as _,
                ),
                _ => (EventKind::Null, 0),
            };

            if self.evelist.borrow_mut().Record_Add_i(
                event.clock() as _,
                event.unit_no(),
                kind as u8,
                value,
            ) {
                Ok(())
            } else {
                Err(AddEventError {})
            }
        }
    }
}

impl<T: Borrow<pxtnEvelist>> EventList for PxToneEventList<T> {
    fn iter(&self) -> Box<dyn Iterator<Item = &dyn GenericEvent>> {
        Box::new(
            EventLinkedList {
                raw: self.evelist.borrow()._start as *const EVERECORD,
            }
            .into_iter()
            .map(|e: &'static EVERECORD| e as &dyn GenericEvent),
        )
    }
}
