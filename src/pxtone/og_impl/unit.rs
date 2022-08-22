use std::{slice, ffi::{CString}};

use pxtone_sys::pxtnUnit;

use crate::interface::unit::Unit;


pub struct PxToneUnit {
    raw: &'static mut pxtnUnit,
}

impl PxToneUnit {
    pub fn new(raw: *mut pxtnUnit) -> Self {
        Self { raw: unsafe { &mut *raw } }
    }
}

impl Unit for PxToneUnit {
    fn selected(&self) -> bool {
        unsafe { self.raw.get_operated() }
    }

    fn set_selected(&mut self, selected: bool) {
        unsafe { self.raw.set_operated(selected); }
    }

    fn muted(&self) -> bool {
        unsafe { !self.raw.get_played() }
    }

    fn set_muted(&mut self, muted: bool) {
        unsafe { self.raw.set_played(!muted); }
    }

    fn name(&self) -> String {
        unsafe {
            if !self.raw.is_name_buf() {
                return "".into();
            }

            let mut len = 0;
            let data = self.raw.get_name_buf(&mut len) as *const u8;
            let arr = slice::from_raw_parts(data, len as usize);
            
            // remove interior NULL bytes
            let mut bytes = Vec::new();
            for b in arr {
                if *b == '\0' as u8 {
                    break;
                }
                bytes.push(*b);
            }

            // add our own NULL byte
            bytes.push('\0' as u8);

            CString::from_vec_with_nul_unchecked(bytes).to_owned().to_string_lossy().into()
        }
    }

    fn set_name(&mut self, name: String) -> Result<(), ()> {
        unsafe {
            if self.raw.set_name_buf(name.as_ptr().cast(), name.len() as i32) {
                Ok(())
            } else {
                Err(())
            }
        }
    }
    
}