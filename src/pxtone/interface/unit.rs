use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use super::{
    event::{EventList, EventListMut},
    service::PxTone,
    woice::{Woices, WoicesMut},
};

pub trait Unit {
    fn selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool);

    fn muted(&self) -> bool;
    fn set_muted(&mut self, muted: bool);

    fn name(&self) -> String;
    fn set_name(&mut self, name: String) -> Result<(), ()>;
}

pub struct Units<'a, U: Unit> {
    _phantom: PhantomData<&'a ()>,
    v: Vec<U>,
}

impl<'a, U: Unit> Units<'a, U> {
    pub fn new<EL: EventList, ELM: EventListMut, W: Woices, WM: WoicesMut>(
        _pxtn: &'a dyn PxTone<
            Unit = U,
            EventList = EL,
            EventListMut = ELM,
            Woices = W,
            WoicesMut = WM,
        >,
        v: Vec<U>,
    ) -> Self {
        Self { _phantom: PhantomData, v }
    }
}

impl<'a, U: Unit> Deref for Units<'a, U> {
    type Target = [U];

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

pub struct UnitsMut<'a, U: Unit> {
    _phantom: PhantomData<&'a ()>,
    v: Vec<U>,
}

impl<'a, U: Unit> UnitsMut<'a, U> {
    pub fn new<EL: EventList, ELM: EventListMut, W: Woices, WM: WoicesMut>(
        _pxtn: &'a mut dyn PxTone<
            Unit = U,
            EventList = EL,
            EventListMut = ELM,
            Woices = W,
            WoicesMut = WM,
        >,
        v: Vec<U>,
    ) -> Self {
        Self { _phantom: PhantomData, v }
    }
}

impl<'a, U: Unit> Deref for UnitsMut<'a, U> {
    type Target = [U];

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<'a, U: Unit> DerefMut for UnitsMut<'a, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}
