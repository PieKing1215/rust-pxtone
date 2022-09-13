use std::{
    borrow::{Borrow, BorrowMut},
    slice,
};

use pxtone_sys::pxtnDelay;

use crate::{
    interface::delay::{AddDelayError, Delay, DelayUnit, Delays, DelaysMut, HasDelays},
    pxtone::util::ZeroToOneF32,
    util::{BoxOrMut, BoxOrRef},
};

use super::service::PxToneService;

impl Delay for pxtnDelay {
    fn group(&self) -> u8 {
        self._group as _
    }

    fn set_group(&mut self, group: u8) {
        self._group = group as _;
    }

    fn frequency(&self) -> DelayUnit {
        match self._unit {
            pxtone_sys::DELAYUNIT_DELAYUNIT_Meas => DelayUnit::Measure(self._freq),
            pxtone_sys::DELAYUNIT_DELAYUNIT_Second => DelayUnit::Second(self._freq),
            _ => DelayUnit::Beat(self._freq),
        }
    }

    fn set_frequency(&mut self, frequency: DelayUnit) {
        match frequency {
            DelayUnit::Beat(f) => {
                self._unit = pxtone_sys::DELAYUNIT_DELAYUNIT_Beat;
                self._freq = f;
            },
            DelayUnit::Measure(f) => {
                self._unit = pxtone_sys::DELAYUNIT_DELAYUNIT_Meas;
                self._freq = f;
            },
            DelayUnit::Second(f) => {
                self._unit = pxtone_sys::DELAYUNIT_DELAYUNIT_Second;
                self._freq = f;
            },
        }
    }

    fn rate(&self) -> ZeroToOneF32 {
        ZeroToOneF32::new(self._rate / 100.0)
    }

    fn set_rate(&mut self, rate: ZeroToOneF32) {
        self._rate = *rate * 100.0;
    }
}

impl<'b, P: Borrow<PxToneService<'b>>> Delays for P {
    type D = pxtnDelay;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::D>> + 'a> {
        let raw = unsafe {
            slice::from_raw_parts(
                self.borrow().raw()._delays,
                self.borrow().raw()._delay_num as usize,
            )
        };
        let v = raw.iter().map(|a| {
            let b: &'a pxtnDelay = unsafe { &**a };
            BoxOrRef::Ref(b)
        });
        Box::new(v)
    }
}

impl<'b, P: BorrowMut<PxToneService<'b>>> DelaysMut for P {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::D>> + 'a> {
        let raw = unsafe {
            slice::from_raw_parts_mut(
                self.borrow_mut().raw_mut()._delays,
                self.borrow_mut().raw_mut()._delay_num as usize,
            )
        };
        let v = raw.iter().map(|a| {
            let b: &'a mut pxtnDelay = unsafe { &mut **a };
            BoxOrMut::Ref(b)
        });
        Box::new(v)
    }

    fn add(
        &mut self,
        group: u8,
        frequency: DelayUnit,
        rate: ZeroToOneF32,
    ) -> Result<BoxOrMut<Self::D>, AddDelayError> {
        let (unit, freq) = match frequency {
            DelayUnit::Beat(f) => (pxtone_sys::DELAYUNIT_DELAYUNIT_Beat, f),
            DelayUnit::Measure(f) => (pxtone_sys::DELAYUNIT_DELAYUNIT_Meas, f),
            DelayUnit::Second(f) => (pxtone_sys::DELAYUNIT_DELAYUNIT_Second, f),
        };

        if unsafe {
            self.borrow_mut()
                .raw_mut()
                .Delay_Add(unit, freq, *rate * 100.0, group as _)
        } {
            let raw = unsafe {
                slice::from_raw_parts_mut(
                    self.borrow_mut().raw_mut()._delays,
                    self.borrow_mut().raw_mut()._delay_num as usize,
                )
            };
            Ok(BoxOrMut::Ref(unsafe { &mut *raw[raw.len() - 1] }))
        } else {
            Err(AddDelayError { group, frequency, rate })
        }
    }

    fn remove(&mut self, index: usize) -> bool {
        unsafe { self.borrow_mut().raw_mut().Delay_Remove(index as _) }
    }
}

impl HasDelays for PxToneService<'_> {
    type Delays = Self;
    type DelaysMut = Self;

    fn delays(&self) -> BoxOrRef<Self::Delays> {
        self.into()
    }

    fn delays_mut(&mut self) -> BoxOrMut<Self::DelaysMut> {
        self.into()
    }
}
