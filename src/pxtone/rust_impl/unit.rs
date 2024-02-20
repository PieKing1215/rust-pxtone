use crate::{interface::unit::{HasUnits, Unit, Units, UnitsMut}, util::{BoxOrMut, BoxOrRef}};

use super::service::RPxTone;

pub struct RPxToneUnit {
    pub(crate) selected: bool,
    pub(crate) muted: bool,
    pub(crate) name: String,
}

impl Units for RPxTone {
    type U = RPxToneUnit;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = crate::util::BoxOrRef<Self::U>> + 'a> {
        Box::new(self.units.iter().map(BoxOrRef::Ref))
    }
}

impl UnitsMut for RPxTone {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = crate::util::BoxOrMut<Self::U>> + 'a> {
        Box::new(self.units.iter_mut().map(BoxOrMut::Ref))
    }

    fn add_new(&mut self) -> Option<crate::util::BoxOrMut<Self::U>> {
        self.units.push(RPxToneUnit {
            selected: false,
            muted: false,
            name: "new unit".into(),
        });
        Some(self.units.last_mut().unwrap().into())
    }

    fn remove(&mut self, index: usize) -> bool {
        if index >= self.units.len() {
            return false;
        }
        self.units.remove(index);
        true
    }
}

impl Unit for RPxToneUnit {
    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    fn muted(&self) -> bool {
        self.muted
    }

    fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) -> Result<(), crate::interface::service::InvalidText> {
        self.name = name;
        Ok(())
    }
}

impl HasUnits for RPxTone {
    type Units = Self;
    type UnitsMut = Self;

    fn units(&self) -> BoxOrRef<Self::Units> {
        BoxOrRef::Ref(self)
    }

    fn units_mut(&mut self) -> BoxOrMut<Self::UnitsMut> {
        BoxOrMut::Ref(self)
    }
}