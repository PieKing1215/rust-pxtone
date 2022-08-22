use pxtone_sys::{EVERECORD, pxtnEvelist};

use crate::interface::event::{EventList, Event, EventListMut, EventKind};

pub struct PxToneEventList {
    raw: &'static pxtnEvelist,
}

impl PxToneEventList {
    pub fn new(raw: *const pxtnEvelist) -> Self {
        Self { raw: unsafe { &*raw } }
    }
}

pub struct PxToneEventListMut {
    raw: &'static mut pxtnEvelist,
}

impl PxToneEventListMut {
    pub fn new(raw: *mut pxtnEvelist) -> Self {
        Self { raw: unsafe { &mut *raw } }
    }
}

pub struct EventLinkedList {
    raw: *const EVERECORD,
}

pub struct IterEventLinkedList {
    current: *const EVERECORD,
}

impl Iterator for IterEventLinkedList {
    type Item = &'static EVERECORD;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            let ret: *const EVERECORD = self.current;
            self.current = unsafe { *self.current }.next;

            Some(unsafe { &*ret })
        }
    }
}

impl IntoIterator for EventLinkedList {
    type Item = &'static EVERECORD;
    type IntoIter = IterEventLinkedList;

    fn into_iter(self) -> Self::IntoIter {
        IterEventLinkedList {
            current: self.raw,
        }
    }
}

pub struct EventLinkedListMut {
    raw: *mut EVERECORD,
}

pub struct IterEventLinkedListMut {
    current: *mut EVERECORD,
}

impl Iterator for IterEventLinkedListMut {
    type Item = &'static mut EVERECORD;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            let ret: *mut EVERECORD = self.current;
            self.current = unsafe { *self.current }.next;

            Some(unsafe { &mut *ret })
        }
    }
}

impl IntoIterator for EventLinkedListMut {
    type Item = &'static mut EVERECORD;
    type IntoIter = IterEventLinkedListMut;

    fn into_iter(self) -> Self::IntoIter {
        IterEventLinkedListMut {
            current: self.raw,
        }
    }
}

impl EventListMut for PxToneEventListMut {
    type E = EVERECORD;
    type I = EventLinkedList;
    type IM = EventLinkedListMut;

    fn events(&self) -> Self::I {
        EventLinkedList {
            raw: (*self.raw)._start,
        }
    }

    fn events_mut(&mut self) -> Self::IM {
        EventLinkedListMut {
            raw: (*self.raw)._start,
        }
    }
    
}

impl EventList for PxToneEventList {
    type E = EVERECORD;
    type I = EventLinkedList;

    fn events(&self) -> Self::I {
        EventLinkedList {
            raw: (*self.raw)._start,
        }
    }
}

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