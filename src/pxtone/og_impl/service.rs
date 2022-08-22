
use std::{convert::TryInto, slice, path::PathBuf, ffi::{CStr, CString}, fs::File};

use pxtone_sys::{pxtnDescriptor, pxtnService, pxtnVOMITPREPARATION, fopen, fclose};

use crate::interface::{moo::{Moo, Fade}, service::PxTone, unit::{Units, UnitsMut}, io::PxToneServiceIO};

use super::{error::Error, unit::PxToneUnit, event::{PxToneEventList, PxToneEventListMut}};
pub struct PxToneService {
    service: pxtnService,
}

impl PxToneService {
    pub fn is_valid(&self) -> bool {
        unsafe { self.service.moo_is_valid_data() }
    }
}

impl PxToneServiceIO for PxToneService {
    type Error = Error;

    fn read_bytes(bytes: &[u8]) -> Result<Self, Self::Error> where Self: Sized {
        let mut serv = unsafe { pxtnService::new() };
        Error::from_raw(unsafe { serv.init() })?;

        let mut descriptor = unsafe { pxtnDescriptor::new() };
        if ! unsafe { descriptor.set_memory_r(bytes as *const _ as *mut std::ffi::c_void, bytes.len() as i32) } {
            return Err(Error::DescR);
        }

        Error::from_raw(unsafe { serv.read(&mut descriptor) })?;

        Ok(Self {
            service: serv,
        })
    }

    fn write_file(&mut self, path: impl Into<PathBuf>) -> Result<Vec<u8>, Self::Error> {
        let path = path.into();
        File::create(&path).map_err(|_| Error::DescW)?;

        let path: PathBuf = path.canonicalize().map_err(|_| Error::DescW)?;
        let fpath = CString::new(path.to_string_lossy().as_bytes()).unwrap();
        
        let file = unsafe { fopen(fpath.as_ptr(), CStr::from_bytes_with_nul_unchecked(b"wb\0").as_ptr()) };

        if file.is_null() {
            return Err(Error::DescW);
        }

        let mut descriptor = unsafe { pxtnDescriptor::new() };
        if ! unsafe { descriptor.set_file_w(file) } {
            return Err(Error::DescW);
        }

        Error::from_raw(unsafe { self.service.write(&mut descriptor, false, 0) })?;

        unsafe { fclose(file) };

        Ok(vec![])
    }
}

impl PxTone for PxToneService {
    type Unit = PxToneUnit;
    type EventList = PxToneEventList;
    type EventListMut = PxToneEventListMut;

    fn units(&self) -> Units<Self::Unit> {
        let raw = unsafe { slice::from_raw_parts(self.service._units, self.service._unit_num.try_into().unwrap()) };
        let v = raw.iter().map(|r| PxToneUnit::new(*r)).collect::<Vec<_>>();
        Units::new(self, v)
    }

    fn units_mut(&mut self) -> UnitsMut<Self::Unit> {
        let raw = unsafe { slice::from_raw_parts(self.service._units, self.service._unit_num.try_into().unwrap()) };
        let v = raw.iter().map(|r| PxToneUnit::new(*r)).collect::<Vec<_>>();
        UnitsMut::new(self, v)
    }

    fn event_list(&self) -> Self::EventList {
        PxToneEventList::new(self.service.evels)
    }

    fn event_list_mut(&mut self) -> Self::EventListMut {
        PxToneEventListMut::new(self.service.evels)
    }
}

impl Moo<Error> for PxToneService {

    fn set_audio_format(&mut self, channels: u8, sample_rate: u32) -> Result<(), Error> {
        if unsafe { self.service.set_destination_quality(channels as i32, sample_rate as i32) } {
            Error::from_raw(unsafe { self.service.tones_ready() })?;
            Ok(())
        }else{
            Err(Error::VOID)
        }
    }

    fn prepare_sample(&mut self) -> Result<(), Error> {

        let prep = pxtnVOMITPREPARATION {
            start_pos_meas: 0,
            start_pos_sample: 0,
            start_pos_float: 0.0,
            meas_end: 0,
            meas_repeat: 0,
            fadein_sec: 0.0,
            flags: pxtone_sys::pxtnVOMITPREPFLAG_loop,
            master_volume: self.service._moo_master_vol,
        };

        if unsafe { self.service.moo_preparation(&prep) } {
            Ok(())
        }else {
            Err(Error::VOID)
        }
    }

    fn sample(&mut self, buffer: &mut [i16]) -> Result<(), Error> {
        if unsafe { self.service.Moo(buffer.as_mut_ptr() as *mut _ as *mut std::ffi::c_void, buffer.len() as i32 * 2) } {
            Ok(())
        }else {
            Err(Error::VOID)
        }
    }

    fn is_done_sampling(&self) -> bool {
        unsafe {
            self.service.moo_is_end_vomit()
        }
    }

    fn now_clock(&self) -> u32 {
        unsafe { self.service.moo_get_now_clock() }.try_into().unwrap_or(0)
    }

    fn end_clock(&self) -> u32 {
        unsafe { self.service.moo_get_end_clock() }.try_into().unwrap_or(0)
    }

    fn set_unit_mute_enabled(&mut self, unit_mute: bool) -> Result<(), Error> {
        if unsafe { self.service.moo_set_mute_by_unit(unit_mute) } {
            Ok(())
        }else {
            Err(Error::INIT)
        }
    }

    fn set_loop(&mut self, should_loop: bool) -> Result<(), Error> {
        if unsafe { self.service.moo_set_loop(should_loop) } {
            Ok(())
        }else {
            Err(Error::INIT)
        }
    }

    fn set_fade(&mut self, fade: Option<Fade>, duration: std::time::Duration) -> Result<(), Error> {
        if unsafe { self.service.moo_set_fade(fade.map_or(0, |f| match f {
            Fade::In => 1,
            Fade::Out => -1,
        }), duration.as_secs_f32()) } {
            Ok(())
        }else {
            Err(Error::INIT)
        }
    }

    fn sampling_offset(&self) -> u32 {
        unsafe {
            self.service.moo_get_sampling_offset()
        }.try_into().unwrap_or(0)
    }

    fn sampling_end(&self) -> u32 {
        unsafe {
            self.service.moo_get_sampling_end()
        }.try_into().unwrap_or(0)
    }

    fn total_samples(&self) -> u32 {
        unsafe {
            self.service.moo_get_total_sample() as u32
        }
    }

    fn set_master_volume(&mut self, volume: f32) -> Result<(), Error> {
        if unsafe { self.service.moo_set_master_volume(volume) } {
            Ok(())
        }else {
            Err(Error::INIT)
        }
    }
}