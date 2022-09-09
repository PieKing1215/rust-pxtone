use crate::util::{BoxOrMut, BoxOrRef};

use super::service::{InvalidText, PxTone};

pub trait Unit {
    fn selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool);

    fn muted(&self) -> bool;
    fn set_muted(&mut self, muted: bool);

    fn name(&self) -> String;
    fn set_name(&mut self, name: String) -> Result<(), InvalidText>;
}

pub trait Units {
    type U: Unit;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::U>> + 'a>;
}

pub trait UnitsMut: Units {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::U>> + 'a>;
}

pub trait HasUnits: PxTone {
    type Units: Units + Sized;
    type UnitsMut: UnitsMut + Sized;

    fn units(&self) -> BoxOrRef<Self::Units>;
    fn units_mut(&mut self) -> BoxOrMut<Self::UnitsMut>;
}
