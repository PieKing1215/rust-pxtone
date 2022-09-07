use std::{borrow::Borrow, convert::Infallible, marker::PhantomData};

use crate::pxtone::util::{BoxOrMut, BoxOrRef};

use super::service::InvalidText;

// TODO: this file has too many similarly named trait definitions in it, split into a couple files

pub enum WoiceType<
    'a,
    VPCM: VoicePCM + 'a + ?Sized,
    VPTV: VoicePTV + 'a + ?Sized,
    VPTN: VoicePTN + 'a + ?Sized,
    VOGGV: VoiceOGGV + 'a + ?Sized,
    PCM: WoicePCM<VPCM> + 'a + ?Sized,
    PTV: WoicePTV<VPTV> + 'a + ?Sized,
    PTN: WoicePTN<VPTN> + 'a + ?Sized,
    OGGV: WoiceOGGV<VOGGV> + 'a + ?Sized,
    BPCM: Borrow<PCM>,
    BPTV: Borrow<PTV>,
    BPTN: Borrow<PTN>,
    BOGGV: Borrow<OGGV>,
> {
    None,
    PCM(BPCM),
    PTV(BPTV),
    PTN(BPTN),
    OGGV(BOGGV),

    // TODO: consider other ways to do this
    // relevant: https://github.com/rust-lang/rust/issues/32739
    /// Implementation detail needed to hold a lifetime.
    #[allow(clippy::type_complexity)]
    _Phantom(
        Infallible,
        PhantomData<&'a (
            &'a VPCM,
            &'a VPTV,
            &'a VPTN,
            &'a VOGGV,
            &'a PCM,
            &'a PTV,
            &'a PTN,
            &'a OGGV,
        )>,
    ),
}

pub type WoiceTypeRef<'a, VPCM, VPTV, VPTN, VOGGV, PCM, PTV, PTN, OGGV> = WoiceType<
    'a,
    VPCM,
    VPTV,
    VPTN,
    VOGGV,
    PCM,
    PTV,
    PTN,
    OGGV,
    BoxOrRef<'a, PCM>,
    BoxOrRef<'a, PTV>,
    BoxOrRef<'a, PTN>,
    BoxOrRef<'a, OGGV>,
>;

pub type WoiceTypeMut<'a, VPCM, VPTV, VPTN, VOGGV, PCM, PTV, PTN, OGGV> = WoiceType<
    'a,
    VPCM,
    VPTV,
    VPTN,
    VOGGV,
    PCM,
    PTV,
    PTN,
    OGGV,
    BoxOrMut<'a, PCM>,
    BoxOrMut<'a, PTV>,
    BoxOrMut<'a, PTN>,
    BoxOrMut<'a, OGGV>,
>;

pub trait Woice {
    type VPCM: VoicePCM;
    type VPTV: VoicePTV;
    type VPTN: VoicePTN;
    type VOGGV: VoiceOGGV;
    type PCM: WoicePCM<Self::VPCM>;
    type PTV: WoicePTV<Self::VPTV>;
    type PTN: WoicePTN<Self::VPTN>;
    type OGGV: WoiceOGGV<Self::VOGGV>;

    fn name(&self) -> String;
    fn set_name(&mut self, name: String) -> Result<(), InvalidText>;

    #[allow(clippy::type_complexity)] // I can't think of a good way to make this less complex
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
    >;
    #[allow(clippy::type_complexity)] // I can't think of a good way to make this less complex
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
    >;
    // fn set_woice_type(&mut self, w_type: WoiceType);
}

pub trait Voice {
    fn basic_key(&self) -> i32;
    fn set_basic_key(&mut self, basic_key: i32);

    fn volume(&self) -> i32;
    fn set_volume(&mut self, volume: i32);

    fn pan(&self) -> i32;
    fn set_pan(&mut self, pan: i32);

    fn tuning(&self) -> f32;
    fn set_tuning(&mut self, tuning: f32);
}

pub trait VoicePCM: Voice {
    /// Should only be 1 or 2
    fn channels(&self) -> u8;

    fn samples_per_second(&self) -> u32;

    /// Should only be 8 or 16
    fn bits_per_sample(&self) -> u8;

    fn sample_buffer(&self) -> &[u8];
}

pub trait PTVCoordinateWavePoint {
    fn x(&self) -> u32;
    fn y(&self) -> i32;
}

pub trait PTVCoordinateWave {
    type Point: PTVCoordinateWavePoint;

    /// Maximum x coordinate, normally 200
    fn resolution(&self) -> u32;

    fn points(&self) -> Vec<&Self::Point>;
}

pub trait PTVOvertoneWaveTone {
    /// Corresponds to the 1,2,3,... in ptVoice
    fn frequency(&self) -> u8;

    /// Normally in the range `-128..=128`
    fn amplitude(&self) -> i16;
}

pub trait PTVOvertoneWave {
    type Tone: PTVOvertoneWaveTone;

    // TODO: might be a better word for this
    fn tones(&self) -> Vec<&Self::Tone>;
}

pub enum PTVWaveType<'a, C: PTVCoordinateWave, O: PTVOvertoneWave> {
    Coordinate(&'a C),
    Overtone(&'a O),
}

pub trait VoicePTV: Voice {
    type CoordinateWave: PTVCoordinateWave;
    type OvertoneWave: PTVOvertoneWave;

    fn wave(&self) -> PTVWaveType<Self::CoordinateWave, Self::OvertoneWave>;
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum PTNWaveType {
    None,
    Sine,
    Saw,
    Rect,
    Random,
    Saw2,
    Rect2,
    Tri,
    Random2,
    Rect3,
    Rect4,
    Rect8,
    Rect16,
    Saw3,
    Saw4,
    Saw6,
    Saw8,
}

impl From<u8> for PTNWaveType {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Sine,
            2 => Self::Saw,
            3 => Self::Rect,
            4 => Self::Random,
            5 => Self::Saw2,
            6 => Self::Rect2,
            7 => Self::Tri,
            8 => Self::Random2,
            9 => Self::Rect3,
            10 => Self::Rect4,
            11 => Self::Rect8,
            12 => Self::Rect16,
            13 => Self::Saw3,
            14 => Self::Saw4,
            15 => Self::Saw6,
            16 => Self::Saw8,
            _ => Self::None,
        }
    }
}

pub trait PTNOscillator {
    /// Shape of the wave
    ///
    /// See the guild in ptNoise
    fn shape(&self) -> PTNWaveType;

    /// Shape of the wave
    ///
    /// See the guild in ptNoise
    fn set_shape(&mut self, shape: PTNWaveType);

    /// Frequency of the oscillator in Hz
    ///
    /// Exactly as it appears in ptNoise
    fn frequency(&self) -> f32;

    /// Frequency of the oscillator in Hz
    ///
    /// Exactly as it appears in ptNoise
    fn set_frequency(&mut self, frequency: f32);

    /// Volume as %. Normally `0.0..=100.0`, but can go >`100.0`
    ///
    /// Exactly as it appears in ptNoise
    fn volume(&self) -> f32;

    /// Volume as %. Normally `0.0..=100.0`, but can go >`100.0`
    ///
    /// Exactly as it appears in ptNoise
    fn set_volume(&mut self, volume: f32);

    /// Phase offset as % of wave period. Normally `0.0..=100.0`, but can go >`100.0`
    ///
    /// Exactly as it appears in ptNoise
    fn offset(&self) -> f32;

    /// Phase offset as % of wave period. Normally `0.0..=100.0`, but can go >`100.0`
    ///
    /// Exactly as it appears in ptNoise
    fn set_offset(&mut self, offset: f32);

    /// `true` if the wave shape should be flipped horizontally
    fn reverse(&self) -> bool;

    /// `true` if the wave shape should be flipped horizontally
    fn set_reverse(&mut self, reverse: bool);
}

pub trait PTNEnvelopePoint {
    /// "x" position, in samples
    fn x(&self) -> u32;

    /// "y" position, as an amplitude `0..=100`
    fn y(&self) -> u8;
}

pub trait PTNUnit {
    type EnvelopePoint: PTNEnvelopePoint;
    type Oscillator: PTNOscillator;

    fn enabled(&self) -> bool;
    fn envelope(&self) -> Vec<&Self::EnvelopePoint>;

    /// Normally within `-100..=100`
    fn pan(&self) -> i8;

    fn osc_main(&self) -> &Self::Oscillator;
    fn osc_frequency(&self) -> &Self::Oscillator;
    fn osc_volume(&self) -> &Self::Oscillator;
}

pub trait VoicePTN: VoicePCM {
    type Unit: PTNUnit;

    /// Number of samples. Capped to 480000 in OG pxtone.
    ///
    /// PTNs are always 44100kHz so you can do `sample_num() / 44100.0` to get the length in seconds
    fn ptn_sample_num(&self) -> u32;

    fn units(&self) -> Vec<&Self::Unit>;
}

pub trait VoiceOGGV: VoicePCM {
    /// Should only be 1 or 2
    fn ogg_channels(&self) -> u8;

    fn ogg_samples_per_second(&self) -> u32;

    fn ogg_sample_num(&self) -> u32;

    fn ogg_data(&self) -> &[u8];
}

pub trait SingleVoice<V: Voice + ?Sized> {
    fn voice(&self) -> &V;
    fn voice_mut(&mut self) -> &mut V;
}

pub trait WoicePCM<V: VoicePCM + ?Sized>: SingleVoice<V> {}

pub trait WoicePTV<V: VoicePTV + ?Sized> {
    fn voices(&self) -> Vec<&V>;
}

pub trait WoicePTN<V: VoicePTN + ?Sized>: SingleVoice<V> {}

pub trait WoiceOGGV<V: VoiceOGGV + ?Sized>: SingleVoice<V> {}

pub trait Woices {
    type W: Woice;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::W>> + 'a>;
}

pub trait WoicesMut: Woices {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::W>> + 'a>;
}
