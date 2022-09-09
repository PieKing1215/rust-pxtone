use std::{
    convert::TryInto,
    ffi::{CStr, CString},
    fs::File,
    path::PathBuf,
    slice,
};

use pxtone_sys::{fclose, fopen, pxtnDescriptor, pxtnEvelist, pxtnService, pxtnVOMITPREPARATION};

use crate::{
    interface::{
        io::PxToneServiceIO,
        moo::{Fade, Moo},
        service::{InvalidText, PxTone},
    },
    pxtone::util::BoxOrMut,
    util::BoxOrRef,
};

use super::{error::Error, event::PxToneEventList};
pub struct PxToneService<'p> {
    service: BoxOrMut<'p, pxtnService>,
}

impl<'p> PxToneService<'p> {
    #[must_use]
    pub fn raw(&self) -> &pxtnService {
        &self.service
    }

    #[must_use]
    pub fn raw_mut(&mut self) -> &mut pxtnService {
        &mut self.service
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        unsafe { self.service.moo_is_valid_data() }
    }
}

impl<'p, T: Into<BoxOrMut<'p, pxtnService>>> From<T> for PxToneService<'p> {
    fn from(service: T) -> Self {
        Self { service: service.into() }
    }
}

impl<'p> PxToneServiceIO for PxToneService<'p> {
    type Error = Error;

    fn read_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let mut serv = unsafe { pxtnService::new() };
        Error::from_raw(unsafe { serv.init_collage(pxtone_sys::pxtnMAX_EVENTNUM as _) })?;

        let mut descriptor = unsafe { pxtnDescriptor::new() };
        if !unsafe {
            descriptor.set_memory_r(
                bytes as *const _ as *mut std::ffi::c_void,
                bytes.len() as i32,
            )
        } {
            return Err(Error::DescR);
        }

        Error::from_raw(unsafe { serv.read(&mut descriptor) })?;

        Ok(serv.into())
    }

    fn write_file(&mut self, path: impl Into<PathBuf>) -> Result<Vec<u8>, Self::Error> {
        let path = path.into();
        File::create(&path).map_err(|_| Error::DescW)?;

        let path: PathBuf = path.canonicalize().map_err(|_| Error::DescW)?;
        let fpath = CString::new(path.to_string_lossy().as_bytes()).unwrap();

        let file = unsafe {
            fopen(
                fpath.as_ptr(),
                CStr::from_bytes_with_nul_unchecked(b"wb\0").as_ptr(),
            )
        };

        if file.is_null() {
            return Err(Error::DescW);
        }

        let mut descriptor = unsafe { pxtnDescriptor::new() };
        if !unsafe { descriptor.set_file_w(file) } {
            return Err(Error::DescW);
        }

        Error::from_raw(unsafe { self.service.write(&mut descriptor, false, 0) })?;

        unsafe { fclose(file) };

        Ok(vec![])
    }
}

impl<'p> PxTone for PxToneService<'p> {
    type Units = Self;
    type UnitsMut = Self;
    type EventList = PxToneEventList<&'p pxtnEvelist>;
    type EventListMut = PxToneEventList<&'p mut pxtnEvelist>;
    type Woices = Self;
    type WoicesMut = Self;
    type Delays = Self;
    type DelaysMut = Self;
    type OverDrives = Self;
    type OverDrivesMut = Self;

    fn beat_num(&self) -> i32 {
        unsafe { (*self.service.master)._beat_num }
    }

    fn set_beat_num(&mut self, beat_num: i32) {
        unsafe { (*self.service.master)._beat_num = beat_num }
    }

    fn beat_tempo(&self) -> f32 {
        unsafe { (*self.service.master)._beat_tempo }
    }

    fn set_beat_tempo(&mut self, beat_tempo: f32) {
        unsafe { (*self.service.master)._beat_tempo = beat_tempo }
    }

    fn beat_clock(&self) -> i32 {
        unsafe { (*self.service.master)._beat_clock }
    }

    fn set_beat_clock(&mut self, beat_clock: i32) {
        unsafe { (*self.service.master).set_beat_clock(beat_clock) }
    }

    fn num_measures(&self) -> i32 {
        unsafe { (*self.service.master)._meas_num }
    }

    fn set_num_measures(&mut self, num_measures: i32) {
        unsafe { (*self.service.master).set_meas_num(num_measures) }
    }

    fn repeat_measure(&self) -> i32 {
        unsafe { (*self.service.master)._repeat_meas }
    }

    fn set_repeat_measure(&mut self, repeat_measure: i32) {
        unsafe { (*self.service.master).set_repeat_meas(repeat_measure) }
    }

    fn last_measure(&self) -> i32 {
        unsafe { (*self.service.master)._last_meas }
    }

    fn set_last_measure(&mut self, last_measure: i32) {
        unsafe { (*self.service.master).set_last_meas(last_measure) }
    }

    fn name(&self) -> String {
        unsafe {
            if !(*self.service.text).is_name_buf() {
                return "".into();
            }

            let mut len = 0;
            let data = (*self.service.text).get_name_buf(&mut len).cast::<u8>();
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
            if (*self.service.text).set_name_buf(name.as_ptr().cast(), name.len() as i32) {
                Ok(())
            } else {
                Err(InvalidText)
            }
        }
    }

    fn comment(&self) -> String {
        unsafe {
            if !(*self.service.text).is_comment_buf() {
                return "".into();
            }

            let mut len = 0;
            let data = (*self.service.text).get_comment_buf(&mut len).cast::<u8>();
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

    fn set_comment(&mut self, comment: String) -> Result<(), InvalidText> {
        unsafe {
            if (*self.service.text).set_comment_buf(comment.as_ptr().cast(), comment.len() as i32) {
                Ok(())
            } else {
                Err(InvalidText)
            }
        }
    }

    fn units(&self) -> BoxOrRef<Self::Units> {
        self.into()
    }

    fn units_mut(&mut self) -> BoxOrMut<Self::UnitsMut> {
        self.into()
    }

    fn event_list(&self) -> BoxOrRef<Self::EventList> {
        PxToneEventList::new(unsafe { &*self.service.evels }).into()
    }

    fn event_list_mut(&mut self) -> BoxOrMut<Self::EventListMut> {
        PxToneEventList::new(unsafe { &mut *self.service.evels }).into()
    }

    fn woices(&self) -> BoxOrRef<Self::Woices> {
        self.into()
    }

    fn woices_mut(&mut self) -> BoxOrMut<Self::WoicesMut> {
        self.into()
    }

    fn delays(&self) -> BoxOrRef<Self::Delays> {
        self.into()
    }

    fn delays_mut(&mut self) -> BoxOrMut<Self::DelaysMut> {
        self.into()
    }

    fn overdrives(&self) -> BoxOrRef<Self::OverDrives> {
        self.into()
    }

    fn overdrives_mut(&mut self) -> BoxOrMut<Self::OverDrivesMut> {
        self.into()
    }
}

impl<'p> Moo<Error> for PxToneService<'p> {
    fn set_audio_format(&mut self, channels: u8, sample_rate: u32) -> Result<(), Error> {
        if unsafe {
            self.service
                .set_destination_quality(channels as i32, sample_rate as i32)
        } {
            Error::from_raw(unsafe { self.service.tones_ready() })?;
            Ok(())
        } else {
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
        } else {
            Err(Error::VOID)
        }
    }

    fn sample(&mut self, buffer: &mut [i16]) -> Result<(), Error> {
        if unsafe {
            self.service
                .Moo(buffer.as_mut_ptr().cast(), buffer.len() as i32 * 2)
        } {
            Ok(())
        } else {
            Err(Error::VOID)
        }
    }

    fn is_done_sampling(&self) -> bool {
        unsafe { self.service.moo_is_end_vomit() }
    }

    fn now_clock(&self) -> u32 {
        unsafe { self.service.moo_get_now_clock() }
            .try_into()
            .unwrap_or(0)
    }

    fn end_clock(&self) -> u32 {
        unsafe { self.service.moo_get_end_clock() }
            .try_into()
            .unwrap_or(0)
    }

    fn set_unit_mute_enabled(&mut self, unit_mute: bool) -> Result<(), Error> {
        if unsafe { self.service.moo_set_mute_by_unit(unit_mute) } {
            Ok(())
        } else {
            Err(Error::INIT)
        }
    }

    fn set_loop(&mut self, should_loop: bool) -> Result<(), Error> {
        if unsafe { self.service.moo_set_loop(should_loop) } {
            Ok(())
        } else {
            Err(Error::INIT)
        }
    }

    fn set_fade(&mut self, fade: Option<Fade>, duration: std::time::Duration) -> Result<(), Error> {
        if unsafe {
            self.service.moo_set_fade(
                fade.map_or(0, |f| match f {
                    Fade::In => 1,
                    Fade::Out => -1,
                }),
                duration.as_secs_f32(),
            )
        } {
            Ok(())
        } else {
            Err(Error::INIT)
        }
    }

    fn sampling_offset(&self) -> u32 {
        unsafe { self.service.moo_get_sampling_offset() }
            .try_into()
            .unwrap_or(0)
    }

    fn sampling_end(&self) -> u32 {
        unsafe { self.service.moo_get_sampling_end() }
            .try_into()
            .unwrap_or(0)
    }

    fn total_samples(&self) -> u32 {
        unsafe { self.service.moo_get_total_sample() as u32 }
    }

    fn set_master_volume(&mut self, volume: f32) -> Result<(), Error> {
        if unsafe { self.service.moo_set_master_volume(volume) } {
            Ok(())
        } else {
            Err(Error::INIT)
        }
    }
}
