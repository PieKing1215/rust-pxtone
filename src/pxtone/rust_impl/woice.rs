use std::{f32::consts::PI, io::Cursor};

use lewton::{inside_ogg::OggStreamReader, VorbisError};

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

    pub(crate) flag_loop: bool,
    pub(crate) flag_smooth: bool,
    pub(crate) flag_beat_fit: bool,

    pub(crate) channels: u8,
    pub(crate) samples_per_second: u32,
    pub(crate) bits_per_sample: u8,
    pub(crate) samples: Vec<f32>,
    pub(crate) sample_num: u32,
    pub(crate) ratio_to_a: f32,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RPxToneVoicePCMError {
    InvalidPCMConfig { bits_per_sample: u8, channels: u8 },
}

impl RPxToneVoicePCM {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::cast_precision_loss)]
    pub fn new(
        basic_key: i32,
        volume: i32,
        pan: i32,
        tuning: f32,
        channels: u8,
        samples_per_second: u32,
        bits_per_sample: u8,
        data: Vec<u8>,
        flag_loop: bool,
        flag_smooth: bool,
        flag_beat_fit: bool,
    ) -> Result<Self, RPxToneVoicePCMError> {
        let sample_num: u32 = data.len() as u32 / (bits_per_sample as u32 / 8 * channels as u32);

        // a woice with 200 samples at 44100Hz is A
        let ratio_to_a = sample_num as f32 / (200.0 * samples_per_second as f32 / 44100.0);
        let semitone_key_offset = (17664 - basic_key) as f32 / 256.0;
        let ratio_to_a = ratio_to_a / 2_f32.powf(semitone_key_offset / 12.0);

        let samples = match (bits_per_sample, channels) {
            (8, 1) => data
                .into_iter()
                .map(|s| s as f32 / u8::MAX as f32 - 0.5)
                .collect(),
            (16, 1) => data
                .chunks_exact(2)
                .map(|a| i16::from_ne_bytes([a[0], a[1]]) as f32 / i16::MAX as f32 / 2.0)
                .collect(),
            //TODO: real stereo
            (8, 2) => data
                .chunks_exact(2)
                .map(|s| s[0] as f32 / u8::MAX as f32 - 0.5)
                .collect(),
            //TODO: real stereo
            (16, 2) => data
                .chunks_exact(4)
                .map(|a| i16::from_ne_bytes([a[0], a[1]]) as f32 / i16::MAX as f32 / 2.0)
                .collect(),
            _ => return Err(RPxToneVoicePCMError::InvalidPCMConfig { bits_per_sample, channels }),
        };

        Ok(Self {
            basic_key,
            volume,
            pan,
            tuning,
            flag_loop,
            flag_smooth,
            flag_beat_fit,
            channels,
            samples_per_second,
            bits_per_sample,
            samples,
            sample_num,
            ratio_to_a,
        })
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

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::inline_always)]
    #[inline(always)] // this function is very hot
    fn sample(&self, cycle: f32) -> f32 {
        let idx = cycle / self.ratio_to_a * self.tuning;

        if self.flag_loop {
            self.samples[(self.samples.len() as f32 * idx) as usize % self.samples.len()]
        } else {
            let i = (self.samples.len() as f32 * idx) as usize;
            if i < self.samples.len() {
                self.samples[i]
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

    pub(crate) samples: Vec<f32>,
    pub(crate) ratio_to_a: f32,
}

impl RPxToneVoicePTV {
    #[must_use]
    pub fn new(
        basic_key: i32,
        volume: i32,
        pan: i32,
        tuning: f32,
        wave: RPxTonePTVWaveType,
    ) -> Self {
        let sample_num = 400;
        let channels = 2;
        let samples_per_second = 44100;
        let bits_per_sample = 16;

        let ratio_to_a = sample_num as f32 / (200.0 * samples_per_second as f32 / 44100.0);
        let semitone_key_offset = (17664 - basic_key) as f32 / 256.0;
        let ratio_to_a = ratio_to_a / 2_f32.powf(semitone_key_offset / 12.0);

        // TODO: stereo
        let samples = (0..sample_num)
            .map(|i| match &wave {
                RPxTonePTVWaveType::Coordinate(c) => {
                    c.sample(i as f32 / sample_num as f32) * volume as f32 / 128.0 / 128.0 / 2.0
                },
                RPxTonePTVWaveType::Overtone(o) => {
                    o.sample(i as f32 / sample_num as f32) * volume as f32 / 128.0 / 2.0
                },
            })
            .collect();

        Self {
            basic_key,
            volume,
            pan,
            tuning,
            wave,
            samples,
            ratio_to_a,
        }
    }
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

impl VoicePCM for RPxToneVoicePTV {
    fn channels(&self) -> u8 {
        2
    }

    fn samples_per_second(&self) -> u32 {
        44100
    }

    fn bits_per_sample(&self) -> u8 {
        16
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::inline_always)]
    #[inline(always)] // this function is very hot
    fn sample(&self, cycle: f32) -> f32 {
        let idx = cycle / self.ratio_to_a * self.tuning;

        self.samples[(self.samples.len() as f32 * idx) as usize % self.samples.len()]
    }
}

pub struct RPxTonePTVCoordinateWave {
    pub(crate) resolution: u32,
    pub(crate) points: Vec<RPxTonePTVCoordinatePoint>,
}

impl RPxTonePTVCoordinateWave {
    pub fn sample(&self, thru: f32) -> f32 {
        let i = (self.resolution as f32 * thru) as u32;

        let mut c = 0;
        while c < self.points.len() {
            if self.points[c].x > i {
                break;
            }
            c += 1;
        }

        let p1;
        let p2;

        if c == self.points.len() {
            p1 = RPxTonePTVCoordinatePoint::new(self.points[c - 1].x, self.points[c - 1].y);
            p2 = RPxTonePTVCoordinatePoint::new(self.resolution, self.points[0].y);
        } else if c > 0 {
            p1 = RPxTonePTVCoordinatePoint::new(self.points[c - 1].x, self.points[c - 1].y);
            p2 = RPxTonePTVCoordinatePoint::new(self.points[c].x, self.points[c].y);
        } else {
            p1 = RPxTonePTVCoordinatePoint::new(self.points[0].x, self.points[0].y);
            p2 = RPxTonePTVCoordinatePoint::new(self.points[0].x, self.points[0].y);
        }

        let w = p2.x - p1.x;
        let i = i - p1.x;
        let h = p2.y - p1.y;

        if i > 0 {
            p1.y as f32 + h as f32 * i as f32 / w as f32
        } else {
            p1.y as f32
        }
    }
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

impl RPxTonePTVCoordinatePoint {
    pub(crate) fn new(x: u32, y: i32) -> Self {
        Self { x, y }
    }
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

impl RPxTonePTVOvertoneWave {
    pub fn sample(&self, thru: f32) -> f32 {
        let mut out = 0.0;
        for t in &self.tones {
            let sss = 2.0 * PI * t.frequency as f32 * thru;
            out += sss.sin() * t.amplitude as f32 / t.frequency as f32 / 128.0;
        }
        out
    }
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

    fn sample(&self, _cycle: f32) -> f32 {
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

    pub(crate) flag_loop: bool,
    pub(crate) flag_smooth: bool,
    pub(crate) flag_beat_fit: bool,

    pub(crate) channels: u8,
    pub(crate) samples_per_second: u32,
    pub(crate) samples: Vec<f32>,
    pub(crate) sample_num: u32,
    pub(crate) ratio_to_a: f32,

    pub(crate) ogg_channels: u8,
    pub(crate) ogg_samples_per_second: u32,
    pub(crate) ogg_sample_num: u32,
    pub(crate) ogg_data: Vec<u8>,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RPxToneVoiceOGGVError {
    InvalidOGGVConfig {
        samples_per_second: u8,
        channels: u8,
    },
    VorbisError(VorbisError),
}

impl RPxToneVoiceOGGV {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::cast_precision_loss)]
    pub fn new(
        basic_key: i32,
        volume: i32,
        pan: i32,
        tuning: f32,
        channels: u8,
        samples_per_second: u32,
        sample_num: u32,
        data: Vec<u8>,
        flag_loop: bool,
        flag_smooth: bool,
        flag_beat_fit: bool,
    ) -> Result<Self, RPxToneVoiceOGGVError> {
        let ogg_data = data;

        // a woice with 200 samples at 44100Hz is A
        let ratio_to_a = sample_num as f32 / (200.0 * samples_per_second as f32 / 44100.0);
        let semitone_key_offset = (17664 - basic_key) as f32 / 256.0;
        let ratio_to_a = ratio_to_a / 2_f32.powf(semitone_key_offset / 12.0);

        let mut ogg_reader = OggStreamReader::new(Cursor::new(&ogg_data))
            .map_err(RPxToneVoiceOGGVError::VorbisError)?;
        let ogg_samples_per_second = ogg_reader.ident_hdr.audio_sample_rate;
        let ogg_channels = ogg_reader.ident_hdr.audio_channels;

        let mut samples = vec![];

        while let Some(raw_samples) = ogg_reader
            .read_dec_packet_itl()
            .map_err(RPxToneVoiceOGGVError::VorbisError)?
        {
            if ogg_channels == 2 {
                //TODO: real stereo
                samples.extend(
                    raw_samples
                        .chunks_exact(2)
                        .map(|a| a[0] as f32 / i16::MAX as f32 / 2.0),
                );
            } else {
                samples.extend(
                    raw_samples
                        .into_iter()
                        .map(|a| a as f32 / i16::MAX as f32 / 2.0),
                );
            }
        }

        Ok(Self {
            basic_key,
            volume,
            pan,
            tuning,
            flag_loop,
            flag_smooth,
            flag_beat_fit,
            channels,
            samples_per_second,
            samples,
            sample_num,
            ratio_to_a,
            ogg_channels,
            ogg_samples_per_second,
            ogg_sample_num: sample_num,
            ogg_data,
        })
    }
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
        8 // TODO: does ogg actually have this?
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::inline_always)]
    #[inline(always)] // this function is very hot
    fn sample(&self, cycle: f32) -> f32 {
        let idx = cycle / self.ratio_to_a * self.tuning;

        if self.flag_loop {
            self.samples[(self.samples.len() as f32 * idx) as usize % self.samples.len()]
        } else {
            let i = (self.samples.len() as f32 * idx) as usize;
            if i < self.samples.len() {
                self.samples[i]
            } else {
                0.0
            }
        }
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

    #[inline]
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
