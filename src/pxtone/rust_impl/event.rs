use std::cmp::Ordering;

use crate::{
    interface::event::{
        BaseEvent, EventGroupNo, EventKey, EventKind, EventList, EventListMut, EventOn,
        EventPanTime, EventPanVolume, EventPorta, EventTuning, EventVelocity, EventVoiceNo,
        EventVolume, GenericEvent, GenericEventKind, GenericEventKindMut, GenericEventKindRef,
        HasEventList, PanValue, TuningValue,
    },
    util::{BoxOrMut, BoxOrRef, ZeroToOneF32},
};

use super::service::RPxTone;

#[derive(Debug, Default)]
pub struct RPxToneEventList {
    pub(crate) events: Vec<RPxToneEvent>,
}

impl RPxToneEventList {
    pub fn sort_events(&mut self) {
        self.events.sort_by(|a, b| {
            match a.clock().cmp(&b.clock()) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => {
                    // TODO: OG pxtone also compares by event type
                    // see pxtnEvelist::_ComparePriority
                    Ordering::Equal
                },
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RPxToneEvent {
    clock: u32,
    unit_no: u8,
    kind: EventKind,
    value: i32,
}

impl BaseEvent for RPxToneEvent {
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

impl GenericEvent for RPxToneEvent {
    type On = Self;
    type Key = Self;
    type PanVolume = Self;
    type Velocity = Self;
    type Volume = Self;
    type Porta = Self;
    type VoiceNo = Self;
    type GroupNo = Self;
    type Tuning = Self;
    type PanTime = Self;

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
    > {
        match self.kind {
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
    > {
        match self.kind {
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

impl EventOn for RPxToneEvent {
    fn length(&self) -> u32 {
        self.value as _
    }

    fn set_length(&mut self, length: u32) {
        self.value = length as _;
    }
}

impl EventKey for RPxToneEvent {
    fn key(&self) -> i32 {
        self.value
    }

    fn set_key(&mut self, key: i32) {
        self.value = key;
    }
}

impl EventPanVolume for RPxToneEvent {
    fn pan_volume(&self) -> PanValue {
        #[allow(clippy::cast_precision_loss)]
        PanValue::new((self.value as f32 / 128.0) * 2.0 - 1.0)
    }

    fn set_pan_volume(&mut self, pan_volume: PanValue) {
        self.value = ((*pan_volume / 2.0 + 0.5) * 128.0) as _;
    }
}

impl EventPorta for RPxToneEvent {
    fn porta(&self) -> u32 {
        self.value as _
    }

    fn set_porta(&mut self, porta: u32) {
        self.value = porta as _;
    }
}

impl EventVelocity for RPxToneEvent {
    fn velocity(&self) -> ZeroToOneF32 {
        #[allow(clippy::cast_precision_loss)]
        ZeroToOneF32::new(self.value as f32 / 128.0)
    }

    fn set_velocity(&mut self, velocity: ZeroToOneF32) {
        self.value = (*velocity * 128.0) as _;
    }
}

impl EventVolume for RPxToneEvent {
    fn volume(&self) -> ZeroToOneF32 {
        #[allow(clippy::cast_precision_loss)]
        ZeroToOneF32::new(self.value as f32 / 128.0)
    }

    fn set_volume(&mut self, volume: ZeroToOneF32) {
        self.value = (*volume * 128.0) as _;
    }
}

impl EventVoiceNo for RPxToneEvent {
    fn voice_no(&self) -> u8 {
        self.value as _
    }

    fn set_voice_no(&mut self, voice_no: u8) {
        self.value = voice_no as _;
    }
}

impl EventGroupNo for RPxToneEvent {
    fn group_no(&self) -> u8 {
        self.value as _
    }

    fn set_group_no(&mut self, group_no: u8) {
        self.value = group_no as _;
    }
}

impl EventTuning for RPxToneEvent {
    fn tuning(&self) -> TuningValue {
        TuningValue::new(f32::from_le_bytes(self.value.to_le_bytes()))
    }

    fn set_tuning(&mut self, tuning: TuningValue) {
        self.value = i32::from_le_bytes(f32::to_le_bytes(*tuning));
    }
}

impl EventPanTime for RPxToneEvent {
    fn pan_time(&self) -> PanValue {
        #[allow(clippy::cast_precision_loss)]
        PanValue::new((self.value as f32 / 128.0) * 2.0 - 1.0)
    }

    fn set_pan_time(&mut self, pan_time: PanValue) {
        self.value = ((*pan_time / 2.0 + 0.5) * 128.0) as _;
    }
}

impl EventList for RPxToneEventList {
    type Event = RPxToneEvent;

    fn iter(&self) -> Box<dyn Iterator<Item = &Self::Event> + '_> {
        Box::new(self.events.iter())
    }
}

impl EventListMut for RPxToneEventList {
    fn iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Event> + '_> {
        Box::new(self.events.iter_mut())
    }

    fn add<E: crate::interface::event::GenericEvent>(
        &mut self,
        event: &E,
    ) -> Result<(), crate::interface::event::AddEventError> {
        let (kind, value) = match event.kind() {
            GenericEventKind::On(e) => (EventKind::On, e.length() as _),
            GenericEventKind::Key(e) => (EventKind::Key, e.key()),
            GenericEventKind::PanVolume(e) => (
                EventKind::PanVolume,
                ((*e.pan_volume() / 2.0 + 0.5) * 128.0) as _,
            ),
            GenericEventKind::Velocity(e) => (EventKind::Velocity, (*e.velocity() * 128.0) as _),
            GenericEventKind::Volume(e) => (EventKind::Volume, (*e.volume() * 128.0) as _),
            GenericEventKind::Porta(e) => (EventKind::Portament, e.porta() as _),
            GenericEventKind::VoiceNo(e) => (EventKind::VoiceNo, e.voice_no() as _),
            GenericEventKind::GroupNo(e) => (EventKind::GroupNo, e.group_no() as _),
            GenericEventKind::Tuning(e) => (
                EventKind::Tuning,
                i32::from_le_bytes(f32::to_le_bytes(*e.tuning())),
            ),
            GenericEventKind::PanTime(e) => (
                EventKind::PanTime,
                ((*e.pan_time() / 2.0 + 0.5) * 128.0) as _,
            ),
            _ => (EventKind::Null, 0),
        };

        let ne = RPxToneEvent {
            clock: event.clock(),
            unit_no: event.unit_no(),
            kind,
            value,
        };

        self.events.push(ne);
        self.sort_events();

        Ok(())
    }
}

impl HasEventList for RPxTone {
    type EventList<'a> = RPxToneEventList where Self: 'a;
    type EventListMut<'a> = RPxToneEventList where Self: 'a;

    fn event_list(&self) -> BoxOrRef<Self::EventList<'_>> {
        BoxOrRef::Ref(&self.event_list)
    }

    fn event_list_mut(&mut self) -> BoxOrMut<Self::EventListMut<'_>> {
        BoxOrMut::Ref(&mut self.event_list)
    }
}
