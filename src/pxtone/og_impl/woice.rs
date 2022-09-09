use std::{ffi::CString, slice};

use pxtone_sys::{
    pxNOISEDESIGN_OSCILLATOR, pxNOISEDESIGN_UNIT, pxtnPOINT, pxtnVOICEUNIT, pxtnVOICEWAVE,
    pxtnWoice,
};

use crate::{
    interface::{
        service::InvalidText,
        woice::{
            HasWoices, PTNEnvelopePoint, PTNOscillator, PTNUnit, PTNWaveType, PTVCoordinateWave,
            PTVCoordinateWavePoint, PTVOvertoneWave, PTVOvertoneWaveTone, PTVWaveType, SingleVoice,
            Voice, VoiceOGGV, VoicePCM, VoicePTN, VoicePTV, Woice, WoiceOGGV, WoicePCM, WoicePTN,
            WoicePTV, WoiceType, WoiceTypeMut, WoiceTypeRef, Woices, WoicesMut,
        },
    },
    pxtone::util::{BoxOrMut, BoxOrRef},
};

use super::service::PxToneService;

impl Woice for pxtnWoice {
    type VPCM = pxtnVOICEUNIT;
    type VPTV = pxtnVOICEUNIT;
    type VPTN = pxtnVOICEUNIT;
    type VOGGV = pxtnVOICEUNIT;
    type PCM = pxtnWoice;
    type PTV = pxtnWoice;
    type PTN = pxtnWoice;
    type OGGV = pxtnWoice;

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

    fn woice_type(
        &self,
    ) -> WoiceTypeRef<
        Self::VPCM,
        Self::VPTV,
        Self::VPTN,
        Self::VOGGV,
        Self::PCM,
        Self::PTV,
        Self::PTN,
        Self::OGGV,
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
    ) -> WoiceTypeMut<
        Self::VPCM,
        Self::VPTV,
        Self::VPTN,
        Self::VOGGV,
        Self::PCM,
        Self::PTV,
        Self::PTN,
        Self::OGGV,
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
    type Point = pxtnPOINT;

    fn resolution(&self) -> u32 {
        self.reso as u32
    }

    fn points(&self) -> Vec<&Self::Point> {
        let slice = unsafe { slice::from_raw_parts(self.points, self.num as usize) };

        slice.iter().collect()
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
    type Tone = pxtnPOINT;

    fn tones(&self) -> Vec<&Self::Tone> {
        let slice = unsafe { slice::from_raw_parts(self.points, self.num as usize) };

        slice.iter().collect()
    }
}

impl VoicePTV for pxtnVOICEUNIT {
    type CoordinateWave = pxtnVOICEWAVE;
    type OvertoneWave = pxtnVOICEWAVE;

    fn wave(&self) -> PTVWaveType<Self::CoordinateWave, Self::OvertoneWave> {
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
        self.freq = frequency;
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
    type EnvelopePoint = pxtnPOINT;
    type Oscillator = pxNOISEDESIGN_OSCILLATOR;

    fn enabled(&self) -> bool {
        self.bEnable
    }

    fn envelope(&self) -> Vec<&Self::EnvelopePoint> {
        let slice = unsafe { slice::from_raw_parts(self.enves, self.enve_num as usize) };

        slice.iter().collect()
    }

    fn pan(&self) -> i8 {
        self.pan as i8
    }

    fn osc_main(&self) -> &Self::Oscillator {
        &self.main
    }

    fn osc_frequency(&self) -> &Self::Oscillator {
        &self.freq
    }

    fn osc_volume(&self) -> &Self::Oscillator {
        &self.volu
    }
}

impl VoicePTN for pxtnVOICEUNIT {
    type Unit = pxNOISEDESIGN_UNIT;

    fn ptn_sample_num(&self) -> u32 {
        unsafe { (*self.p_ptn)._smp_num_44k as u32 }
    }

    fn units(&self) -> Vec<&Self::Unit> {
        let ptn = unsafe { &*self.p_ptn };
        let slice = unsafe { slice::from_raw_parts(ptn._units, ptn._unit_num as usize) };

        slice.iter().collect()
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

impl SingleVoice<pxtnVOICEUNIT> for pxtnWoice {
    fn voice(&self) -> &pxtnVOICEUNIT {
        let voices = unsafe { slice::from_raw_parts(self._voices, self._voice_num as usize) };
        &voices[0]
    }

    fn voice_mut(&mut self) -> &mut pxtnVOICEUNIT {
        let voices = unsafe { slice::from_raw_parts_mut(self._voices, self._voice_num as usize) };
        &mut voices[0]
    }
}

impl WoicePCM<pxtnVOICEUNIT> for pxtnWoice {}

impl WoicePTV<pxtnVOICEUNIT> for pxtnWoice {
    fn voices(&self) -> Vec<&pxtnVOICEUNIT> {
        let voices = unsafe { slice::from_raw_parts(self._voices, self._voice_num as usize) };
        let mut v = Vec::new();
        for ele in voices {
            v.push(ele as &pxtnVOICEUNIT);
        }
        v
    }
}

impl WoicePTN<pxtnVOICEUNIT> for pxtnWoice {}

impl WoiceOGGV<pxtnVOICEUNIT> for pxtnWoice {}

impl Woices for PxToneService<'_> {
    type W = pxtnWoice;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::W>> + 'a> {
        let slice =
            unsafe { slice::from_raw_parts(self.raw()._woices, self.raw()._woice_num as usize) };
        Box::new(
            slice
                .iter()
                .map(|w| BoxOrRef::Ref(unsafe { &**w } as &Self::W)),
        )
    }
}

impl WoicesMut for PxToneService<'_> {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::W>> + 'a> {
        let slice = unsafe {
            slice::from_raw_parts_mut(self.raw_mut()._woices, self.raw_mut()._woice_num as usize)
        };
        Box::new(
            slice
                .iter_mut()
                .map(|w| BoxOrMut::Ref(unsafe { &mut **w } as &mut Self::W)),
        )
    }
}

impl HasWoices for PxToneService<'_> {
    type Woices = Self;
    type WoicesMut = Self;

    fn woices(&self) -> BoxOrRef<Self::Woices> {
        self.into()
    }

    fn woices_mut(&mut self) -> BoxOrMut<Self::WoicesMut> {
        self.into()
    }
}
