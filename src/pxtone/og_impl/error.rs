use core::fmt;
use std::ffi::CStr;

use pxtone_sys::{pxtnERR, pxtnError_get_string};

#[derive(Debug, Copy, Clone)]
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
}

impl Error {
    fn from_i32(n: i32) -> Option<Self> {
        match n {
            1 => Some(Self::VOID),
            2 => Some(Self::INIT),
            3 => Some(Self::FATAL),
            4 => Some(Self::AntiOpreation),
            5 => Some(Self::DenyBeatclock),
            6 => Some(Self::DescW),
            7 => Some(Self::DescR),
            8 => Some(Self::DescBroken),
            9 => Some(Self::FmtNew),
            10 => Some(Self::FmtUnknown),
            11 => Some(Self::InvCode),
            12 => Some(Self::InvData),
            13 => Some(Self::Memory),
            14 => Some(Self::MooInit),
            15 => Some(Self::Ogg),
            16 => Some(Self::OggNoSupported),
            17 => Some(Self::Param),
            18 => Some(Self::PcmConvert),
            19 => Some(Self::PcmUnknown),
            20 => Some(Self::PtnBuild),
            21 => Some(Self::PtnInit),
            22 => Some(Self::PtvNoSupported),
            23 => Some(Self::TooMuchEvent),
            24 => Some(Self::WoiceFull),
            25 => Some(Self::X1XIgnore),
            26 => Some(Self::X3XAddTuning),
            27 => Some(Self::X3XKey),
            _ => None,
        }
    }

    fn to_i32(self) -> i32 {
        self as _
    }

    pub fn from_raw(value: pxtnERR) -> Result<(), Error> {
        // iirc this cast was needed on linux because pxtnERR was compiled as u32 (?)
        #[allow(clippy::unnecessary_cast)]
        Self::from_i32(value as i32).map_or(Ok(()), Err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let msg = CStr::from_ptr(pxtnError_get_string(self.to_i32() as pxtnERR))
                .to_str()
                .unwrap();
            write!(f, "{}", msg)
        }
    }
}
