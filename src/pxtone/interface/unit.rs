use crate::util::{BoxOrMut, BoxOrRef};

use super::service::InvalidText;

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

pub trait HasUnits {
    type Units: Units + Sized;
    type UnitsMut: UnitsMut + Sized; // this type could be moved into Units, but I don't think it really improves anything

    fn units(&self) -> BoxOrRef<Self::Units>;
    fn units_mut(&mut self) -> BoxOrMut<Self::UnitsMut>;
}

/// Some compile fail tests to make sure mutability works correctly
///
/// ```compile_fail
/// use pxtone::interface::unit::{HasUnits, Units, Unit, UnitsMut};
/// fn _f<T: HasUnits>(has_units: &mut T) {
///     for mut bu in has_units.units().iter() {
///         let _ = bu.muted();
///         let u: &mut dyn Unit = &mut *bu;
///         let _ = u.muted();
///     }
/// }
/// ```
///
/// ```compile_fail
/// use pxtone::interface::unit::{HasUnits, Units, Unit, UnitsMut};
/// fn _f<T: HasUnits>(has_units: &mut T) {
///     for mut bu in has_units.units_mut().iter() {
///         let _ = bu.muted();
///         let u: &mut dyn Unit = &mut *bu;
///         let _ = u.muted();
///     }
/// }
/// ```
///
/// ```compile_fail
/// use pxtone::interface::unit::{HasUnits, Units, Unit, UnitsMut};
/// fn _f<T: HasUnits>(has_units: &mut T) {
///     for mut bu in has_units.units_mut().iter_mut() {
///         let _ = bu.muted();
///         bu.set_muted(false);
///         let u: &mut dyn Unit = &mut *bu;
///         let _ = u.muted();
///         u.set_muted(false);
///
///         let m = has_units.units_mut();
///     }
/// }
/// ```
#[cfg(doctest)]
pub struct TestCheckMut;

#[cfg(test)]
mod tests {
    use crate::interface::unit::{HasUnits, Unit, Units, UnitsMut};

    #[test]
    fn iter() {
        fn _f<T: HasUnits>(has_units: &T) {
            for bu in has_units.units().iter() {
                let _ = bu.muted();
                let u: &dyn Unit = &*bu;
                let _ = u.muted();
            }
        }
    }

    #[test]
    fn iter_mut() {
        fn _f<T: HasUnits>(has_units: &mut T) {
            for bu in has_units.units().iter() {
                let _ = bu.muted();
                let u: &dyn Unit = &*bu;
                let _ = u.muted();
            }

            for bu in has_units.units_mut().iter() {
                let _ = bu.muted();
                let u: &dyn Unit = &*bu;
                let _ = u.muted();
            }

            for mut bu in has_units.units_mut().iter_mut() {
                let _ = bu.muted();
                bu.set_muted(false);
                let u: &mut dyn Unit = &mut *bu;
                let _ = u.muted();
                u.set_muted(false);
            }
        }
    }
}
