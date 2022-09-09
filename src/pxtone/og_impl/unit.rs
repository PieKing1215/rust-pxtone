use std::{ffi::CString, slice};

use pxtone_sys::pxtnUnit;

use crate::{
    interface::{
        service::InvalidText,
        unit::{Unit, Units, UnitsMut},
    },
    util::{BoxOrMut, BoxOrRef},
};

use super::service::PxToneService;

impl Unit for pxtnUnit {
    fn selected(&self) -> bool {
        unsafe { self.get_operated() }
    }

    fn set_selected(&mut self, selected: bool) {
        unsafe {
            self.set_operated(selected);
        }
    }

    fn muted(&self) -> bool {
        unsafe { !self.get_played() }
    }

    fn set_muted(&mut self, muted: bool) {
        unsafe {
            self.set_played(!muted);
        }
    }

    fn name(&self) -> String {
        unsafe {
            if !self.is_name_buf() {
                return "".into();
            }

            let mut len = 0;
            let data = self.get_name_buf(&mut len).cast::<u8>();
            let arr = slice::from_raw_parts(data, len as usize);

            // remove interior NULL bytes
            let mut bytes = Vec::new();
            for b in arr {
                if *b == b'\0' {
                    break;
                }
                bytes.push(*b);
            }

            // add our own NULL byte
            bytes.push(b'\0');

            CString::from_vec_with_nul_unchecked(bytes)
                .to_string_lossy()
                .into()
        }
    }

    fn set_name(&mut self, name: String) -> Result<(), InvalidText> {
        unsafe {
            if self.set_name_buf(name.as_ptr().cast(), name.len() as i32) {
                Ok(())
            } else {
                Err(InvalidText)
            }
        }
    }
}

impl Units for PxToneService<'_> {
    type U = pxtnUnit;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::U>> + 'a> {
        let slice =
            unsafe { slice::from_raw_parts(self.raw()._units, self.raw()._unit_num as usize) };
        Box::new(
            slice
                .iter()
                .map(|u| BoxOrRef::Ref(unsafe { &**u } as &Self::U)),
        )
    }
}

impl UnitsMut for PxToneService<'_> {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = crate::util::BoxOrMut<Self::U>> + 'a> {
        let slice = unsafe {
            slice::from_raw_parts_mut(self.raw_mut()._units, self.raw_mut()._unit_num as usize)
        };
        Box::new(
            slice
                .iter_mut()
                .map(|u| BoxOrMut::Ref(unsafe { &mut **u } as &mut Self::U)),
        )
    }
}
