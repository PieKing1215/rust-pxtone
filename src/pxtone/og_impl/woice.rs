
use std::{marker::PhantomData, slice, ffi::CString, borrow::{Borrow, BorrowMut}};

use pxtone_sys::pxtnWoice;

use crate::{interface::woice::{Woices, Woice, WoicesMut}, pxtone::util::{BoxOrRef, BoxOrMut}};

impl Woice for pxtnWoice {
    fn name(&self) -> String {
        unsafe {
            if !self.is_name_buf() {
                return "".into();
            }

            let mut len = 0;
            let data = self.get_name_buf(&mut len) as *const u8;
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
            if self.set_name_buf(name.as_ptr().cast(), name.len() as i32) {
                Ok(())
            } else {
                Err(())
            }
        }
    }
}

pub struct PxToneWoices<'p, T: Borrow<pxtnWoice>> {
    _phantom: PhantomData<&'p ()>,
    woices: Vec<T>,
}

impl<'p, T: Borrow<pxtnWoice>> PxToneWoices<'p, T> {
    pub fn new(woices: Vec<T>) -> Self {
        Self {
            _phantom: PhantomData,
            woices,
        }
    }
}

impl<'p, T: Borrow<pxtnWoice>> Woices for PxToneWoices<'p, T> {
    type W = pxtnWoice;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::W>> + 'a> {
        let v = (&self.woices).into_iter().map(|a| {
            let b: &'a pxtnWoice = a.borrow();
            BoxOrRef::Ref(b)
        });
        Box::new(v)
    }
}

impl<'p, T: BorrowMut<pxtnWoice>> WoicesMut for PxToneWoices<'p, T> {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::W>> + 'a> {
        let v = (&mut self.woices).into_iter().map(|a| {
            let b: &'a mut pxtnWoice = a.borrow_mut();
            BoxOrMut::Ref(b)
        });
        Box::new(v)
    }
}
