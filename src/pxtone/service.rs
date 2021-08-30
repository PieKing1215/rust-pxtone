
use pxtone_sys::{pxtnDescriptor, pxtnService, pxtnVOMITPREPARATION};

use crate::error::Error;


pub struct PxTone {
    service: pxtnService,
}

pub enum Fade {
    In, Out,
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

        Ok(Self {
            service: serv,
        })
    }

    pub fn set_audio_format(&mut self, channels: u8, sample_rate: u32) -> Result<(), Error> {
        if unsafe { self.service.set_destination_quality(channels as i32, sample_rate as i32) } {
            Error::from_raw(unsafe { self.service.tones_ready() })?;
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
            flags: pxtone_sys::pxtnVOMITPREPFLAG_loop,
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

    // getters / setters

    pub fn is_valid(&self) -> bool {
        unsafe { self.service.moo_is_valid_data() }
    }

    pub fn is_done_sampling(&self) -> bool {
        unsafe {
            self.service.moo_is_end_vomit()
        }
    }

    // maybe use a u31 type since sys get_now_clock returns an i32 that is always positive
    pub fn get_now_clock(&self) -> i32 {
        unsafe { self.service.moo_get_now_clock() }
    }

    // maybe use a u31 type since sys get_end_clock returns an i32 that is always positive
    pub fn get_end_clock(&self) -> i32 {
        unsafe { self.service.moo_get_end_clock() }
    }

    pub fn set_unit_mute_enabled(&mut self, unit_mute: bool) -> Result<(), Error> {
        if unsafe { self.service.moo_set_mute_by_unit(unit_mute) } {
            Ok(())
        }else {
            Err(Error::INIT)
        }
    }

    pub fn set_loop(&mut self, should_loop: bool) -> Result<(), Error> {
        if unsafe { self.service.moo_set_loop(should_loop) } {
            Ok(())
        }else {
            Err(Error::INIT)
        }
    }

    pub fn set_fade(&mut self, fade: Option<Fade>, duration: std::time::Duration) -> Result<(), Error> {
        if unsafe { self.service.moo_set_fade(fade.map_or(0, |f| match f {
            Fade::In => 1,
            Fade::Out => -1,
        }), duration.as_secs_f32()) } {
            Ok(())
        }else {
            Err(Error::INIT)
        }
    }

    pub fn get_sampling_offset(&self) -> i32 {
        unsafe {
            self.service.moo_get_sampling_offset()
        }
    }

    pub fn get_sampling_end(&self) -> i32 {
        unsafe {
            self.service.moo_get_sampling_end()
        }
    }

    pub fn get_total_samples(&self) -> u32 {
        unsafe {
            self.service.moo_get_total_sample() as u32
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) -> Result<(), Error> {
        if unsafe { self.service.moo_set_master_volume(volume) } {
            Ok(())
        }else {
            Err(Error::INIT)
        }
    }
}