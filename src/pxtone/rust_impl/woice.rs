use crate::{
    interface::{
        service::InvalidText,
        woice::{
            HasWoices, PTNEnvelopePoint, PTNOscillator, PTNUnit, PTNWaveType, PTVCoordinateWave,
            PTVCoordinateWavePoint, PTVOvertoneWave, PTVOvertoneWaveTone, PTVWaveType, SingleVoice,
            Voice, VoiceOGGV, VoicePCM, VoicePTN, VoicePTV, Woice, WoiceOGGV, WoicePCM, WoicePTN,
            WoicePTV, WoiceTypeMut, WoiceTypeRef, Woices, WoicesMut,
        },
    },
    util::{BoxOrMut, BoxOrRef},
};

use super::service::RPxTone;

pub struct RPxToneWoice {
    pub(crate) name: String,
    pub(crate) woice_type: RPxToneWoiceType,
}

pub enum RPxToneWoiceType {
    PCM(RPxToneWoicePCM),
    PTV(RPxToneWoicePTV),
    PTN(RPxToneWoicePTN),
    OGGV(RPxToneWoiceOGGV),
}

pub struct RPxToneWoicePCM {
    pub(crate) voice: RPxToneVoicePCM,
}

impl SingleVoice<RPxToneVoicePCM> for RPxToneWoicePCM {
    fn voice(&self) -> &RPxToneVoicePCM {
        &self.voice
    }

    fn voice_mut(&mut self) -> &mut RPxToneVoicePCM {
        &mut self.voice
    }
}

impl WoicePCM<RPxToneVoicePCM> for RPxToneWoicePCM {}

pub struct RPxToneVoicePCM {
    pub(crate) basic_key: i32,
    pub(crate) volume: i32,
    pub(crate) pan: i32,
    pub(crate) tuning: f32,

    pub(crate) flags: u32, // TODO: bitflags
    pub(crate) channels: u8,
    pub(crate) samples_per_second: u32,
    pub(crate) bits_per_sample: u8,
    pub(crate) data: Vec<u8>,
    pub(crate) sample_num: u32,
}

impl RPxToneVoicePCM {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        basic_key: i32,
        volume: i32,
        pan: i32,
        tuning: f32,
        channels: u8,
        samples_per_second: u32,
        bits_per_sample: u8,
        data: Vec<u8>,
        flags: u32,
    ) -> Self {
        let sample_num: u32 = data.len() as u32 / (bits_per_sample as u32 / 8 * channels as u32);

        Self {
            basic_key,
            volume,
            pan,
            tuning,
            channels,
            samples_per_second,
            bits_per_sample,
            data,
            flags,
            sample_num,
        }
    }
}

impl Voice for RPxToneVoicePCM {
    fn basic_key(&self) -> i32 {
        self.basic_key
    }

    fn set_basic_key(&mut self, basic_key: i32) {
        self.basic_key = basic_key;
    }

    fn volume(&self) -> i32 {
        self.volume
    }

    fn set_volume(&mut self, volume: i32) {
        self.volume = volume;
    }

    fn pan(&self) -> i32 {
        self.pan
    }

    fn set_pan(&mut self, pan: i32) {
        self.pan = pan;
    }

    fn tuning(&self) -> f32 {
        self.tuning
    }

    fn set_tuning(&mut self, tuning: f32) {
        self.tuning = tuning;
    }
}

impl VoicePCM for RPxToneVoicePCM {
    fn channels(&self) -> u8 {
        self.channels
    }

    fn samples_per_second(&self) -> u32 {
        self.samples_per_second
    }

    fn bits_per_sample(&self) -> u8 {
        self.bits_per_sample
    }

    fn sample(&self, cycle: f32) -> f32 {
        let ratio_to_a =
            self.sample_num as f32 / (200.0 * self.samples_per_second as f32 / 44100.0);
        let idx = cycle / ratio_to_a;
        if self.flags & 0x01 > 0 {
            // loop flag
            self.data[(self.data.len() as f64 * idx as f64) as usize % self.data.len()] as f32
                / 256.0
                - 0.5
        } else {
            let i = (self.data.len() as f64 * idx as f64) as usize;
            if i < self.data.len() {
                self.data[i] as f32 / 256.0 - 0.5
            } else {
                0.0
            }
        }
    }
}

pub struct RPxToneWoicePTV {
    pub(crate) voices: Vec<RPxToneVoicePTV>,
}

impl WoicePTV<RPxToneVoicePTV> for RPxToneWoicePTV {
    fn voices(&self) -> Vec<&RPxToneVoicePTV> {
        self.voices.iter().collect()
    }
}

pub struct RPxToneVoicePTV {
    pub(crate) basic_key: i32,
    pub(crate) volume: i32,
    pub(crate) pan: i32,
    pub(crate) tuning: f32,

    pub(crate) wave: RPxTonePTVWaveType,
}

pub enum RPxTonePTVWaveType {
    Coordinate(RPxTonePTVCoordinateWave),
    Overtone(RPxTonePTVOvertoneWave),
}

impl Voice for RPxToneVoicePTV {
    fn basic_key(&self) -> i32 {
        self.basic_key
    }

    fn set_basic_key(&mut self, basic_key: i32) {
        self.basic_key = basic_key;
    }

    fn volume(&self) -> i32 {
        self.volume
    }

    fn set_volume(&mut self, volume: i32) {
        self.volume = volume;
    }

    fn pan(&self) -> i32 {
        self.pan
    }

    fn set_pan(&mut self, pan: i32) {
        self.pan = pan;
    }

    fn tuning(&self) -> f32 {
        self.tuning
    }

    fn set_tuning(&mut self, tuning: f32) {
        self.tuning = tuning;
    }
}

impl VoicePTV for RPxToneVoicePTV {
    type CoordinateWave = RPxTonePTVCoordinateWave;
    type OvertoneWave = RPxTonePTVOvertoneWave;

    fn wave(&self) -> PTVWaveType<Self::CoordinateWave, Self::OvertoneWave> {
        match &self.wave {
            RPxTonePTVWaveType::Coordinate(c) => PTVWaveType::Coordinate(c),
            RPxTonePTVWaveType::Overtone(o) => PTVWaveType::Overtone(o),
        }
    }
}

pub struct RPxTonePTVCoordinateWave {
    pub(crate) resolution: u32,
    pub(crate) points: Vec<RPxTonePTVCoordinatePoint>,
}

impl PTVCoordinateWave for RPxTonePTVCoordinateWave {
    type Point = RPxTonePTVCoordinatePoint;

    fn resolution(&self) -> u32 {
        self.resolution
    }

    fn points(&self) -> Vec<&Self::Point> {
        self.points.iter().collect()
    }
}

pub struct RPxTonePTVCoordinatePoint {
    pub(crate) x: u32,
    pub(crate) y: i32,
}

impl PTVCoordinateWavePoint for RPxTonePTVCoordinatePoint {
    fn x(&self) -> u32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

pub struct RPxTonePTVOvertoneWave {
    pub(crate) tones: Vec<RPxTonePTVOvertoneWaveTone>,
}

impl PTVOvertoneWave for RPxTonePTVOvertoneWave {
    type Tone = RPxTonePTVOvertoneWaveTone;

    fn tones(&self) -> Vec<&Self::Tone> {
        self.tones.iter().collect()
    }
}

pub struct RPxTonePTVOvertoneWaveTone {
    pub(crate) frequency: u8,
    pub(crate) amplitude: i16,
}

impl PTVOvertoneWaveTone for RPxTonePTVOvertoneWaveTone {
    fn frequency(&self) -> u8 {
        self.frequency
    }

    fn amplitude(&self) -> i16 {
        self.amplitude
    }
}

pub struct RPxToneWoicePTN {
    pub(crate) voice: RPxToneVoicePTN,
}

impl SingleVoice<RPxToneVoicePTN> for RPxToneWoicePTN {
    fn voice(&self) -> &RPxToneVoicePTN {
        &self.voice
    }

    fn voice_mut(&mut self) -> &mut RPxToneVoicePTN {
        &mut self.voice
    }
}

impl WoicePTN<RPxToneVoicePTN> for RPxToneWoicePTN {}

pub struct RPxToneVoicePTN {
    pub(crate) basic_key: i32,
    pub(crate) volume: i32,
    pub(crate) pan: i32,
    pub(crate) tuning: f32,

    pub(crate) channels: u8,
    pub(crate) samples_per_second: u32,
    pub(crate) bits_per_sample: u8,

    pub(crate) ptn_sample_num: u32,
    pub(crate) ptn_units: Vec<RPxTonePTNUnit>,
}

impl Voice for RPxToneVoicePTN {
    fn basic_key(&self) -> i32 {
        self.basic_key
    }

    fn set_basic_key(&mut self, basic_key: i32) {
        self.basic_key = basic_key;
    }

    fn volume(&self) -> i32 {
        self.volume
    }

    fn set_volume(&mut self, volume: i32) {
        self.volume = volume;
    }

    fn pan(&self) -> i32 {
        self.pan
    }

    fn set_pan(&mut self, pan: i32) {
        self.pan = pan;
    }

    fn tuning(&self) -> f32 {
        self.tuning
    }

    fn set_tuning(&mut self, tuning: f32) {
        self.tuning = tuning;
    }
}

impl VoicePCM for RPxToneVoicePTN {
    fn channels(&self) -> u8 {
        self.channels
    }

    fn samples_per_second(&self) -> u32 {
        self.samples_per_second
    }

    fn bits_per_sample(&self) -> u8 {
        self.bits_per_sample
    }

    fn sample(&self, cycle: f32) -> f32 {
        todo!()
    }
}

impl VoicePTN for RPxToneVoicePTN {
    type Unit = RPxTonePTNUnit;

    fn ptn_sample_num(&self) -> u32 {
        self.ptn_sample_num
    }

    fn units(&self) -> Vec<&Self::Unit> {
        self.ptn_units.iter().collect()
    }
}

pub struct RPxTonePTNUnit {
    pub(crate) enabled: bool,
    pub(crate) pan: i8,
    pub(crate) envelope: Vec<RPxTonePTNEnvelopePoint>,
    pub(crate) osc_main: RPxTonePTNOscillator,
    pub(crate) osc_frequency: RPxTonePTNOscillator,
    pub(crate) osc_volume: RPxTonePTNOscillator,
}

impl PTNUnit for RPxTonePTNUnit {
    type EnvelopePoint = RPxTonePTNEnvelopePoint;
    type Oscillator = RPxTonePTNOscillator;

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn envelope(&self) -> Vec<&Self::EnvelopePoint> {
        self.envelope.iter().collect()
    }

    fn pan(&self) -> i8 {
        self.pan
    }

    fn osc_main(&self) -> &Self::Oscillator {
        &self.osc_main
    }

    fn osc_frequency(&self) -> &Self::Oscillator {
        &self.osc_frequency
    }

    fn osc_volume(&self) -> &Self::Oscillator {
        &self.osc_volume
    }
}

pub struct RPxTonePTNEnvelopePoint {
    pub(crate) x: u32,
    pub(crate) y: u8,
}

impl PTNEnvelopePoint for RPxTonePTNEnvelopePoint {
    fn x(&self) -> u32 {
        self.x
    }

    fn y(&self) -> u8 {
        self.y
    }
}

pub struct RPxTonePTNOscillator {
    pub(crate) shape: PTNWaveType,
    pub(crate) frequency: f32,
    pub(crate) volume: f32,
    pub(crate) offset: f32,
    pub(crate) reverse: bool,
}

impl PTNOscillator for RPxTonePTNOscillator {
    fn shape(&self) -> PTNWaveType {
        self.shape
    }

    fn set_shape(&mut self, shape: PTNWaveType) {
        self.shape = shape;
    }

    fn frequency(&self) -> f32 {
        self.frequency
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    fn volume(&self) -> f32 {
        self.volume
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    fn offset(&self) -> f32 {
        self.offset
    }

    fn set_offset(&mut self, offset: f32) {
        self.offset = offset;
    }

    fn reverse(&self) -> bool {
        self.reverse
    }

    fn set_reverse(&mut self, reverse: bool) {
        self.reverse = reverse;
    }
}

pub struct RPxToneWoiceOGGV {
    pub(crate) voice: RPxToneVoiceOGGV,
}

impl SingleVoice<RPxToneVoiceOGGV> for RPxToneWoiceOGGV {
    fn voice(&self) -> &RPxToneVoiceOGGV {
        &self.voice
    }

    fn voice_mut(&mut self) -> &mut RPxToneVoiceOGGV {
        &mut self.voice
    }
}

impl WoiceOGGV<RPxToneVoiceOGGV> for RPxToneWoiceOGGV {}

pub struct RPxToneVoiceOGGV {
    pub(crate) basic_key: i32,
    pub(crate) volume: i32,
    pub(crate) pan: i32,
    pub(crate) tuning: f32,

    pub(crate) channels: u8,
    pub(crate) samples_per_second: u32,
    pub(crate) bits_per_sample: u8,

    pub(crate) ogg_channels: u8,
    pub(crate) ogg_samples_per_second: u32,
    pub(crate) ogg_sample_num: u32,
    pub(crate) ogg_data: Vec<u8>,
}

impl Voice for RPxToneVoiceOGGV {
    fn basic_key(&self) -> i32 {
        self.basic_key
    }

    fn set_basic_key(&mut self, basic_key: i32) {
        self.basic_key = basic_key;
    }

    fn volume(&self) -> i32 {
        self.volume
    }

    fn set_volume(&mut self, volume: i32) {
        self.volume = volume;
    }

    fn pan(&self) -> i32 {
        self.pan
    }

    fn set_pan(&mut self, pan: i32) {
        self.pan = pan;
    }

    fn tuning(&self) -> f32 {
        self.tuning
    }

    fn set_tuning(&mut self, tuning: f32) {
        self.tuning = tuning;
    }
}

impl VoicePCM for RPxToneVoiceOGGV {
    fn channels(&self) -> u8 {
        self.channels
    }

    fn samples_per_second(&self) -> u32 {
        self.samples_per_second
    }

    fn bits_per_sample(&self) -> u8 {
        self.bits_per_sample
    }

    fn sample(&self, cycle: f32) -> f32 {
        todo!()
    }
}

impl VoiceOGGV for RPxToneVoiceOGGV {
    fn ogg_channels(&self) -> u8 {
        self.ogg_channels
    }

    fn ogg_samples_per_second(&self) -> u32 {
        self.ogg_samples_per_second
    }

    fn ogg_sample_num(&self) -> u32 {
        self.ogg_sample_num
    }

    fn ogg_data(&self) -> &[u8] {
        &self.ogg_data
    }
}

impl Woice for RPxToneWoice {
    type VPCM = RPxToneVoicePCM;
    type VPTV = RPxToneVoicePTV;
    type VPTN = RPxToneVoicePTN;
    type VOGGV = RPxToneVoiceOGGV;
    type PCM = RPxToneWoicePCM;
    type PTV = RPxToneWoicePTV;
    type PTN = RPxToneWoicePTN;
    type OGGV = RPxToneWoiceOGGV;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) -> Result<(), InvalidText> {
        self.name = name;
        Ok(())
    }

    fn woice_type(
        &self,
    ) -> crate::interface::woice::WoiceTypeRef<
        Self::VPCM,
        Self::VPTV,
        Self::VPTN,
        Self::VOGGV,
        Self::PCM,
        Self::PTV,
        Self::PTN,
        Self::OGGV,
    > {
        match &self.woice_type {
            RPxToneWoiceType::PCM(w) => WoiceTypeRef::PCM(w.into()),
            RPxToneWoiceType::PTV(w) => WoiceTypeRef::PTV(w.into()),
            RPxToneWoiceType::PTN(w) => WoiceTypeRef::PTN(w.into()),
            RPxToneWoiceType::OGGV(w) => WoiceTypeRef::OGGV(w.into()),
        }
    }

    fn woice_type_mut(
        &mut self,
    ) -> crate::interface::woice::WoiceTypeMut<
        Self::VPCM,
        Self::VPTV,
        Self::VPTN,
        Self::VOGGV,
        Self::PCM,
        Self::PTV,
        Self::PTN,
        Self::OGGV,
    > {
        match &mut self.woice_type {
            RPxToneWoiceType::PCM(w) => WoiceTypeMut::PCM(w.into()),
            RPxToneWoiceType::PTV(w) => WoiceTypeMut::PTV(w.into()),
            RPxToneWoiceType::PTN(w) => WoiceTypeMut::PTN(w.into()),
            RPxToneWoiceType::OGGV(w) => WoiceTypeMut::OGGV(w.into()),
        }
    }
}

impl Woices for RPxTone {
    type W = RPxToneWoice;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::W>> + 'a> {
        Box::new(self.woices.iter().map(BoxOrRef::Ref))
    }
}

impl WoicesMut for RPxTone {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::W>> + 'a> {
        Box::new(self.woices.iter_mut().map(BoxOrMut::Ref))
    }
}

impl HasWoices for RPxTone {
    type Woices = Self;
    type WoicesMut = Self;

    fn woices(&self) -> BoxOrRef<Self::Woices> {
        BoxOrRef::Ref(self)
    }

    fn woices_mut(&mut self) -> BoxOrMut<Self::WoicesMut> {
        BoxOrMut::Ref(self)
    }
}
