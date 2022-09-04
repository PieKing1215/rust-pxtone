use std::{
    borrow::{Borrow, BorrowMut},
    ffi::CString,
    marker::PhantomData,
    slice,
};

use pxtone_sys::{
    pxNOISEDESIGN_OSCILLATOR, pxNOISEDESIGN_UNIT, pxtnPOINT, pxtnVOICEUNIT, pxtnVOICEWAVE,
    pxtnWoice,
};

use crate::{
    interface::woice::{
        PTNEnvelopePoint, PTNOscillator, PTNUnit, PTNWaveType, PTVCoordinateWave,
        PTVCoordinateWavePoint, PTVOvertoneWave, PTVOvertoneWaveTone, PTVWaveType, SingleVoice,
        Voice, VoiceOGGV, VoicePCM, VoicePTN, VoicePTV, Woice, WoiceOGGV, WoicePCM, WoicePTN,
        WoicePTV, WoiceType, Woices, WoicesMut,
    },
    pxtone::util::{BoxOrMut, BoxOrRef},
};

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

            CString::from_vec_with_nul_unchecked(bytes)
                .to_owned()
                .to_string_lossy()
                .into()
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

    fn woice_type(
        &self,
    ) -> WoiceType<
        BoxOrRef<dyn WoicePCM>,
        BoxOrRef<dyn WoicePTV>,
        BoxOrRef<dyn WoicePTN>,
        BoxOrRef<dyn WoiceOGGV>,
    > {
        match self._type {
            pxtone_sys::pxtnWOICETYPE_pxtnWOICE_PCM => WoiceType::PCM(BoxOrRef::Ref(self)),
            pxtone_sys::pxtnWOICETYPE_pxtnWOICE_PTV => WoiceType::PTV(BoxOrRef::Ref(self)),
            pxtone_sys::pxtnWOICETYPE_pxtnWOICE_PTN => WoiceType::PTN(BoxOrRef::Ref(self)),
            pxtone_sys::pxtnWOICETYPE_pxtnWOICE_OGGV => WoiceType::OGGV(BoxOrRef::Ref(self)),
            _ => WoiceType::None,
        }
    }

    fn woice_type_mut(
        &mut self,
    ) -> WoiceType<
        BoxOrMut<dyn WoicePCM>,
        BoxOrMut<dyn WoicePTV>,
        BoxOrMut<dyn WoicePTN>,
        BoxOrMut<dyn WoiceOGGV>,
    > {
        match self._type {
            pxtone_sys::pxtnWOICETYPE_pxtnWOICE_PCM => WoiceType::PCM(BoxOrMut::Ref(self)),
            pxtone_sys::pxtnWOICETYPE_pxtnWOICE_PTV => WoiceType::PTV(BoxOrMut::Ref(self)),
            pxtone_sys::pxtnWOICETYPE_pxtnWOICE_PTN => WoiceType::PTN(BoxOrMut::Ref(self)),
            pxtone_sys::pxtnWOICETYPE_pxtnWOICE_OGGV => WoiceType::OGGV(BoxOrMut::Ref(self)),
            _ => WoiceType::None,
        }
    }
}

impl Voice for pxtnVOICEUNIT {
    /// Default is 0x4500
    fn basic_key(&self) -> i32 {
        self.basic_key
    }

    /// Default is 0x4500
    fn set_basic_key(&mut self, basic_key: i32) {
        self.basic_key = basic_key;
    }

    /// Default is 128
    fn volume(&self) -> i32 {
        self.volume
    }

    /// Default is 128
    fn set_volume(&mut self, volume: i32) {
        self.volume = volume;
    }

    /// Default is 64
    fn pan(&self) -> i32 {
        self.pan
    }

    /// Default is 64
    fn set_pan(&mut self, pan: i32) {
        self.pan = pan;
    }

    /// Default is 1.0
    fn tuning(&self) -> f32 {
        self.tuning
    }

    /// Default is 1.0
    fn set_tuning(&mut self, tuning: f32) {
        self.tuning = tuning;
    }
}

impl VoicePCM for pxtnVOICEUNIT {
    fn channels(&self) -> u8 {
        unsafe { (*self.p_pcm)._ch as u8 }
    }

    fn samples_per_second(&self) -> u32 {
        unsafe { (*self.p_pcm)._sps as u32 }
    }

    fn bits_per_sample(&self) -> u8 {
        unsafe { (*self.p_pcm)._bps as u8 }
    }

    fn sample_buffer(&self) -> &[u8] {
        let pcm = unsafe { &*self.p_pcm };
        let size = (pcm._smp_head + pcm._smp_body + pcm._smp_tail) * pcm._ch * pcm._bps / 8;
        unsafe { slice::from_raw_parts(pcm._p_smp, size as usize) }
    }
}

impl PTVCoordinateWavePoint for pxtnPOINT {
    fn x(&self) -> u32 {
        self.x as u32
    }

    fn y(&self) -> i32 {
        self.y
    }
}

impl PTVCoordinateWave for pxtnVOICEWAVE {
    fn resolution(&self) -> u32 {
        self.reso as u32
    }

    fn points(&self) -> Vec<&dyn PTVCoordinateWavePoint> {
        let slice = unsafe { slice::from_raw_parts(self.points, self.num as usize) };

        let mut v = Vec::new();
        for p in slice {
            v.push(p as &dyn PTVCoordinateWavePoint);
        }

        v
    }
}

impl PTVOvertoneWaveTone for pxtnPOINT {
    fn frequency(&self) -> u8 {
        self.x as u8
    }

    fn amplitude(&self) -> i16 {
        self.y as i16
    }
}

impl PTVOvertoneWave for pxtnVOICEWAVE {
    fn tones(&self) -> Vec<&dyn PTVOvertoneWaveTone> {
        let slice = unsafe { slice::from_raw_parts(self.points, self.num as usize) };

        let mut v = Vec::new();
        for p in slice {
            v.push(p as &dyn PTVOvertoneWaveTone);
        }

        v
    }
}

impl VoicePTV for pxtnVOICEUNIT {
    fn wave(&self) -> PTVWaveType {
        if self.type_ == pxtone_sys::pxtnVOICETYPE_pxtnVOICE_Coodinate {
            PTVWaveType::Coordinate(&self.wave)
        } else {
            PTVWaveType::Overtone(&self.wave)
        }
    }
}

impl PTNEnvelopePoint for pxtnPOINT {
    fn x(&self) -> u32 {
        self.x as u32
    }

    fn y(&self) -> u8 {
        self.y as u8
    }
}

impl PTNOscillator for pxNOISEDESIGN_OSCILLATOR {
    fn shape(&self) -> PTNWaveType {
        (self.type_ as u8).into()
    }

    fn frequency(&self) -> f32 {
        self.freq
    }

    fn volume(&self) -> f32 {
        self.volume
    }

    fn offset(&self) -> f32 {
        self.offset
    }

    fn reverse(&self) -> bool {
        self.b_rev
    }

    fn set_shape(&mut self, shape: PTNWaveType) {
        self.type_ = shape as u8 as _;
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.freq = frequency
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    fn set_offset(&mut self, offset: f32) {
        self.offset = offset;
    }

    fn set_reverse(&mut self, reverse: bool) {
        self.b_rev = reverse;
    }
}

impl PTNUnit for pxNOISEDESIGN_UNIT {
    fn enabled(&self) -> bool {
        self.bEnable
    }

    fn envelope(&self) -> Vec<&dyn PTNEnvelopePoint> {
        let slice = unsafe { slice::from_raw_parts(self.enves, self.enve_num as usize) };

        let mut v = Vec::new();
        for p in slice {
            v.push(p as &dyn PTNEnvelopePoint);
        }

        v
    }

    fn pan(&self) -> i8 {
        self.pan as i8
    }

    fn osc_main(&self) -> &dyn PTNOscillator {
        &self.main
    }

    fn osc_frequency(&self) -> &dyn PTNOscillator {
        &self.freq
    }

    fn osc_volume(&self) -> &dyn PTNOscillator {
        &self.volu
    }
}

impl VoicePTN for pxtnVOICEUNIT {
    fn ptn_sample_num(&self) -> u32 {
        unsafe { (*self.p_ptn)._smp_num_44k as u32 }
    }

    fn units(&self) -> Vec<&dyn PTNUnit> {
        let ptn = unsafe { &*self.p_ptn };
        let slice = unsafe { slice::from_raw_parts(ptn._units, ptn._unit_num as usize) };

        let mut v = Vec::new();
        for p in slice {
            v.push(p as &dyn PTNUnit);
        }

        v
    }
}

impl VoiceOGGV for pxtnVOICEUNIT {
    fn ogg_data(&self) -> &[u8] {
        let ogg = unsafe { &*self.p_oggv };
        unsafe { slice::from_raw_parts(ogg._p_data.cast(), ogg._size as usize) }
    }

    fn ogg_channels(&self) -> u8 {
        unsafe { (*self.p_oggv)._ch as u8 }
    }

    fn ogg_samples_per_second(&self) -> u32 {
        unsafe { (*self.p_oggv)._sps2 as u32 }
    }

    fn ogg_sample_num(&self) -> u32 {
        unsafe { (*self.p_oggv)._smp_num as u32 }
    }
}

impl<'a> SingleVoice<'a, dyn VoicePCM> for pxtnWoice {
    fn voice<'b>(&'b self) -> &'b (dyn VoicePCM + 'a) {
        let voices = unsafe { slice::from_raw_parts(self._voices, self._voice_num as usize) };
        &voices[0]
    }

    fn voice_mut<'b>(&'b mut self) -> &'b mut (dyn VoicePCM + 'a) {
        let voices = unsafe { slice::from_raw_parts_mut(self._voices, self._voice_num as usize) };
        &mut voices[0]
    }
}

impl<'a> SingleVoice<'a, dyn VoicePTN> for pxtnWoice {
    fn voice<'b>(&'b self) -> &'b (dyn VoicePTN + 'a) {
        let voices = unsafe { slice::from_raw_parts(self._voices, self._voice_num as usize) };
        &voices[0]
    }

    fn voice_mut<'b>(&'b mut self) -> &'b mut (dyn VoicePTN + 'a) {
        let voices = unsafe { slice::from_raw_parts_mut(self._voices, self._voice_num as usize) };
        &mut voices[0]
    }
}

impl<'a> SingleVoice<'a, dyn VoiceOGGV> for pxtnWoice {
    fn voice<'b>(&'b self) -> &'b (dyn VoiceOGGV + 'a) {
        let voices = unsafe { slice::from_raw_parts(self._voices, self._voice_num as usize) };
        &voices[0]
    }

    fn voice_mut<'b>(&'b mut self) -> &'b mut (dyn VoiceOGGV + 'a) {
        let voices = unsafe { slice::from_raw_parts_mut(self._voices, self._voice_num as usize) };
        &mut voices[0]
    }
}

impl WoicePCM<'_> for pxtnWoice {}

impl WoicePTV for pxtnWoice {
    fn voices(&self) -> Vec<&dyn VoicePTV> {
        let voices = unsafe { slice::from_raw_parts(self._voices, self._voice_num as usize) };
        let mut v = Vec::new();
        for ele in voices {
            v.push(ele as &dyn VoicePTV);
        }
        v
    }
}

impl WoicePTN<'_> for pxtnWoice {}

impl WoiceOGGV<'_> for pxtnWoice {}

pub struct PxToneWoices<'p, T: Borrow<pxtnWoice>> {
    _phantom: PhantomData<&'p ()>,
    woices: Vec<T>,
}

impl<'p, T: Borrow<pxtnWoice>> PxToneWoices<'p, T> {
    pub fn new(woices: Vec<T>) -> Self {
        Self { _phantom: PhantomData, woices }
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
