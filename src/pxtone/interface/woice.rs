use std::{borrow::Borrow, convert::Infallible, marker::PhantomData};

use crate::pxtone::util::{BoxOrMut, BoxOrRef};

use super::service::InvalidText;

// TODO: this file has too many similarly named trait definitions in it, split into a couple files

pub enum WoiceType<
    'a,
    PCM: Borrow<dyn WoicePCM<'a> + 'a>,
    PTV: Borrow<dyn WoicePTV + 'a>,
    PTN: Borrow<dyn WoicePTN<'a> + 'a>,
    OGGV: Borrow<dyn WoiceOGGV<'a> + 'a>,
> {
    None,
    PCM(PCM),
    PTV(PTV),
    PTN(PTN),
    OGGV(OGGV),

    // TODO: consider other ways to do this
    // relevant: https://github.com/rust-lang/rust/issues/32739
    /// Implementation detail needed to hold a lifetime.
    _Phantom(Infallible, PhantomData<&'a ()>),
}

pub type WoiceTypeRef<'a> = WoiceType<
    'a,
    BoxOrRef<'a, dyn WoicePCM<'a>>,
    BoxOrRef<'a, dyn WoicePTV>,
    BoxOrRef<'a, dyn WoicePTN<'a>>,
    BoxOrRef<'a, dyn WoiceOGGV<'a>>,
>;

pub type WoiceTypeMut<'a> = WoiceType<
    'a,
    BoxOrMut<'a, dyn WoicePCM<'a>>,
    BoxOrMut<'a, dyn WoicePTV>,
    BoxOrMut<'a, dyn WoicePTN<'a>>,
    BoxOrMut<'a, dyn WoiceOGGV<'a>>,
>;

pub trait Woice {
    fn name(&self) -> String;
    fn set_name(&mut self, name: String) -> Result<(), InvalidText>;

    fn woice_type(&self) -> WoiceTypeRef;
    fn woice_type_mut(&mut self) -> WoiceTypeMut;
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
    /// Maximum x coordinate, normally 200
    fn resolution(&self) -> u32;

    fn points(&self) -> Vec<&dyn PTVCoordinateWavePoint>;
}

pub trait PTVOvertoneWaveTone {
    /// Corresponds to the 1,2,3,... in ptVoice
    fn frequency(&self) -> u8;

    /// Normally in the range `-128..=128`
    fn amplitude(&self) -> i16;
}

pub trait PTVOvertoneWave {
    // TODO: might be a better word for this
    fn tones(&self) -> Vec<&dyn PTVOvertoneWaveTone>;
}

pub enum PTVWaveType<'a> {
    Coordinate(&'a dyn PTVCoordinateWave),
    Overtone(&'a dyn PTVOvertoneWave),
}

pub trait VoicePTV: Voice {
    fn wave(&self) -> PTVWaveType;
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
    fn enabled(&self) -> bool;
    fn envelope(&self) -> Vec<&dyn PTNEnvelopePoint>;

    /// Normally within `-100..=100`
    fn pan(&self) -> i8;

    fn osc_main(&self) -> &dyn PTNOscillator;
    fn osc_frequency(&self) -> &dyn PTNOscillator;
    fn osc_volume(&self) -> &dyn PTNOscillator;
}

pub trait VoicePTN: VoicePCM {
    /// Number of samples. Capped to 480000 in OG pxtone.
    ///
    /// PTNs are always 44100kHz so you can do `sample_num() / 44100.0` to get the length in seconds
    fn ptn_sample_num(&self) -> u32;

    fn units(&self) -> Vec<&dyn PTNUnit>;
}

pub trait VoiceOGGV: VoicePCM {
    /// Should only be 1 or 2
    fn ogg_channels(&self) -> u8;

    fn ogg_samples_per_second(&self) -> u32;

    fn ogg_sample_num(&self) -> u32;

    fn ogg_data(&self) -> &[u8];
}

pub trait SingleVoice<'a, V: Voice + 'a + ?Sized> {
    fn voice(&self) -> &V;
    fn voice_mut(&mut self) -> &mut V;
}

pub trait WoicePCM<'a>: SingleVoice<'a, dyn VoicePCM> {}

pub trait WoicePTV {
    fn voices(&self) -> Vec<&dyn VoicePTV>;
}

pub trait WoicePTN<'a>: SingleVoice<'a, dyn VoicePTN> {}

pub trait WoiceOGGV<'a>: SingleVoice<'a, dyn VoiceOGGV> {}

// you can implement `IntoIterator` for `&dyn Woices` but not for `<W: Woices> &W`
// and the impl for `&dyn Woices` isn't very useful becuase you have to manually cast it anyway

pub trait Woices {
    type W: Woice;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::W>> + 'a>;
}

pub trait WoicesMut: Woices {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::W>> + 'a>;
}
