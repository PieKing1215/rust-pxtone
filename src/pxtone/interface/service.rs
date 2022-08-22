

use super::{unit::{Unit, Units, UnitsMut}, event::{EventList, EventListMut}};

pub trait PxTone {
    type Unit: Unit + Sized;
    type EventList: EventList + Sized;
    type EventListMut: EventListMut + Sized;

    fn units(&self) -> Units<Self::Unit>;
    fn units_mut(&mut self) -> UnitsMut<Self::Unit>;

    fn event_list(&self) -> Self::EventList;
    fn event_list_mut(&mut self) -> Self::EventListMut;
}