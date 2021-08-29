
use pxtone_sys::{pxtnDescriptor, pxtnService, pxtnVOMITPREPARATION};

use crate::error::Error;

pub struct PxTone {
    service: pxtnService,
}

impl PxTone {
    pub fn read_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut serv = unsafe { pxtnService::new() };
        Error::from_raw(unsafe { serv.init() })?;

        let mut descriptor = unsafe { pxtnDescriptor::new() };
        if ! unsafe { descriptor.set_memory_r(bytes as *const _ as *mut std::ffi::c_void, bytes.len() as i32) } {
            return Err(Error::VOID);
        }

        Error::from_raw(unsafe { serv.read(&mut descriptor) })?;
        Error::from_raw(unsafe { serv.tones_ready() })?;

        Ok(Self {
            service: serv,
        })
    }

    pub fn set_audio_format(&mut self, channels: u8, sample_rate: u32) -> Result<(), Error> {
        if unsafe { self.service.set_destination_quality(channels as i32, sample_rate as i32) } {
            Ok(())
        }else{
            Err(Error::VOID)
        }
    }

    pub fn prepare_sample(&mut self) -> Result<(), Error> {

        let prep = pxtnVOMITPREPARATION {
            start_pos_meas: 0,
            start_pos_sample: 0,
            start_pos_float: 0.0,
            meas_end: 0,
            meas_repeat: 0,
            fadein_sec: 0.0,
            flags: 0,
            master_volume: 0.5,
        };

        if unsafe { self.service.moo_preparation(&prep) } {
            Ok(())
        }else {
            Err(Error::VOID)
        }
    }

    pub fn sample(&mut self, buffer: &mut [i16]) -> Result<(), Error> {
        if unsafe { self.service.Moo(buffer.as_mut_ptr() as *mut _ as *mut std::ffi::c_void, buffer.len() as i32 * 2) } {
            Ok(())
        }else {
            Err(Error::VOID)
        }
    }

    pub fn get_total_samples(&mut self) -> u32 {
        unsafe {
            self.service.moo_get_total_sample() as u32
        }
    }
}