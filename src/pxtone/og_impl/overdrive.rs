use std::{
    borrow::{Borrow, BorrowMut},
    slice,
};

use pxtone_sys::pxtnOverDrive;

use crate::{
    interface::overdrive::{
        AddOverDriveError, HasOverDrives, OverDAmp, OverDCut, OverDrive, OverDrives, OverDrivesMut,
    },
    util::{BoxOrMut, BoxOrRef},
};

use super::service::PxToneService;

impl OverDrive for pxtnOverDrive {
    fn group(&self) -> u8 {
        self._group as _
    }

    fn set_group(&mut self, group: u8) {
        self._group = group as _;
    }

    fn cut(&self) -> OverDCut {
        OverDCut::new(self._cut_f / 100.0)
    }

    fn set_cut(&mut self, cut: OverDCut) {
        self._cut_f = *cut * 100.0;
    }

    fn amp(&self) -> OverDAmp {
        OverDAmp::new(self._amp_f)
    }

    fn set_amp(&mut self, amp: OverDAmp) {
        self._amp_f = *amp;
    }
}

impl<'b, P: Borrow<PxToneService<'b>>> OverDrives for P {
    type O = pxtnOverDrive;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::O>> + 'a> {
        let raw = unsafe {
            slice::from_raw_parts(
                self.borrow().raw()._ovdrvs,
                self.borrow().raw()._ovdrv_num as usize,
            )
        };
        let v = raw.iter().map(|a| {
            let b: &'a pxtnOverDrive = unsafe { &**a };
            BoxOrRef::Ref(b)
        });
        Box::new(v)
    }
}

impl<'b, P: BorrowMut<PxToneService<'b>>> OverDrivesMut for P {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::O>> + 'a> {
        let raw = unsafe {
            slice::from_raw_parts_mut(
                self.borrow_mut().raw_mut()._ovdrvs,
                self.borrow_mut().raw_mut()._ovdrv_num as usize,
            )
        };
        let v = raw.iter().map(|a| {
            let b: &'a mut pxtnOverDrive = unsafe { &mut **a };
            BoxOrMut::Ref(b)
        });
        Box::new(v)
    }

    fn add(&mut self, group: u8, cut: OverDCut, amp: OverDAmp) -> Result<(), AddOverDriveError> {
        if unsafe {
            self.borrow_mut()
                .raw_mut()
                .OverDrive_Add(*cut * 100.0, *amp, group as _)
        } {
            Ok(())
        } else {
            Err(AddOverDriveError { group, cut, amp })
        }
    }

    fn remove(&mut self, index: usize) -> bool {
        unsafe { self.borrow_mut().raw_mut().OverDrive_Remove(index as _) }
    }
}

impl HasOverDrives for PxToneService<'_> {
    type OverDrives = Self;
    type OverDrivesMut = Self;

    fn overdrives(&self) -> BoxOrRef<Self::OverDrives> {
        self.into()
    }

    fn overdrives_mut(&mut self) -> BoxOrMut<Self::OverDrivesMut> {
        self.into()
    }
}
