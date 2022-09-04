use std::borrow::{Borrow, BorrowMut};

use pxtone_sys::{EVERECORD, pxtnEvelist};

use crate::interface::event::{EventList, Event, EventListMut, EventKind};

impl Event for EVERECORD {
    fn kind(&self) -> EventKind {
        self.kind.into()
    }

    fn unit_no(&self) -> u8 {
        self.unit_no
    }

    fn set_unit_no(&mut self, u: u8) {
        self.unit_no = u;
    }

    fn value(&self) -> i32 {
        self.value
    }

    fn set_value(&mut self, v: i32) {
        self.value = v;
    }

    fn clock(&self) -> i32 {
        self.clock
    }

    fn set_clock(&mut self, c: i32) {
        self.clock = c;
    }
}

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
        IterEventLinkedList {
            current: self.raw,
        }
    }
}

impl<T: BorrowMut<pxtnEvelist>> EventListMut for PxToneEventList<T> {
    type IM = EventLinkedList<*mut EVERECORD>;

    fn iter_mut(&mut self) -> Self::IM {
        EventLinkedList {
            raw: self.evelist.borrow_mut()._start,
        }
    }
    
}

impl<T: Borrow<pxtnEvelist>> EventList for PxToneEventList<T> {
    type E = EVERECORD;
    type I = EventLinkedList<*const EVERECORD>;

    fn iter(&self) -> Self::I {
        EventLinkedList {
            raw: self.evelist.borrow()._start,
        }
    }
}