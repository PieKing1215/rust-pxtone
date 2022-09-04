use core::fmt;
use std::ffi::CStr;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use pxtone_sys::{pxtnERR, pxtnError_get_string};

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Error {
    VOID = 1,
    INIT = 2,
    FATAL = 3,
    AntiOpreation = 4,
    DenyBeatclock = 5,
    DescW = 6,
    DescR = 7,
    DescBroken = 8,
    FmtNew = 9,
    FmtUnknown = 10,
    InvCode = 11,
    InvData = 12,
    Memory = 13,
    MooInit = 14,
    Ogg = 15,
    OggNoSupported = 16,
    Param = 17,
    PcmConvert = 18,
    PcmUnknown = 19,
    PtnBuild = 20,
    PtnInit = 21,
    PtvNoSupported = 22,
    TooMuchEvent = 23,
    WoiceFull = 24,
    X1XIgnore = 25,
    X3XAddTuning = 26,
    X3XKey = 27,
    Num = 28,
}

impl Error {
    pub fn from_raw(value: pxtnERR) -> Result<(), Error> {
        Self::from_i32(value as i32).map_or(Ok(()), Err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let msg = self
                .to_i32()
                .map(|err| {
                    CStr::from_ptr(pxtnError_get_string(err as pxtnERR))
                        .to_str()
                        .unwrap()
                })
                .unwrap_or("???");
            write!(f, "{}", msg)
        }
    }
}
