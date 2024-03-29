use std::{collections::HashMap, ops::Deref};

use crate::{
    interface::{
        event::{
            BaseEvent, EventKey, EventOn, EventPanVolume, EventPorta, EventTuning, EventVelocity,
            EventVoiceNo, EventVolume, GenericEvent, GenericEventKind, PanValue, TuningValue,
        },
        moo::{AsMooRef, Moo},
        service::PxTone,
        woice::{VoicePCM, Woice, WoiceType},
    },
    util::{BoxOrMut, ZeroToOneF32},
};

use super::service::RPxTone;

pub struct RPxToneMoo<'a> {
    pxtone: &'a RPxTone,
    channels: u8,
    sample_rate: u32,

    smp: u32,
    last_clock: f32,
    last_sample_clock_secs: f32,

    unit_data: HashMap<u8, UnitData>,

    master_volume: f32,
}

struct UnitData {
    on: Option<UnitOnData>,
    /// current output key value (including porta)
    key_now: i32,
    /// key at the start of the note (porta)
    key_start: i32,
    /// offset from key_start to end value (porta)
    key_margin: i32,
    volume: ZeroToOneF32,
    velocity: ZeroToOneF32,
    woice: u8,
    tuning: TuningValue,
    porta: u32,
    porta_start: u32,
    pan_volume: PanValue,
}

pub const DEFAULT_KEY: i32 = 24576;

#[allow(clippy::derivable_impls)]
impl Default for UnitData {
    fn default() -> Self {
        Self {
            on: None,
            key_now: DEFAULT_KEY,
            key_start: DEFAULT_KEY,
            key_margin: 0,
            volume: ZeroToOneF32::new(104.0 / 128.0),
            velocity: ZeroToOneF32::new(104.0 / 128.0),
            woice: 0,
            tuning: TuningValue::new(1.0),
            porta: 0,
            porta_start: 0,
            pan_volume: PanValue::center(),
        }
    }
}

struct UnitOnData {
    start: u32,
    length: u32,
    /// Needs to be double precision to prevent artifacts
    /// TODO: see if this impacts performance
    cycle: f64,
}

impl Deref for RPxToneMoo<'_> {
    type Target = RPxTone;

    fn deref(&self) -> &Self::Target {
        self.pxtone
    }
}

#[derive(Debug)]
pub enum RPxToneMooError {}

impl AsMooRef for RPxTone {
    type M<'a> = RPxToneMoo<'a> where Self: 'a;

    fn as_moo_ref(&self) -> BoxOrMut<Self::M<'_>> {
        BoxOrMut::Box(Box::new(RPxToneMoo {
            pxtone: self,
            channels: 2,
            sample_rate: 44100,

            smp: 0,
            last_clock: 0.0,
            last_sample_clock_secs: 0.0,
            unit_data: HashMap::new(),

            master_volume: 1.0,
        }))
    }
}

impl<'a> Moo<'a> for RPxToneMoo<'a> {
    type Error = RPxToneMooError;

    fn set_audio_format(&mut self, channels: u8, sample_rate: u32) -> Result<(), RPxToneMooError> {
        self.channels = channels;
        self.sample_rate = sample_rate;
        Ok(())
    }

    fn prepare_sample(&mut self) -> Result<(), RPxToneMooError> {
        Ok(())
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::unreadable_literal)]
    #[allow(clippy::too_many_lines)]
    fn sample(&mut self, buffer: &mut [i16]) -> Result<(), RPxToneMooError> {
        profiling::scope!("sample");
        // println!("buf {}", buffer.len());
        let evs = self.pxtone.event_list.events.iter().collect::<Vec<_>>();

        let smooth_smps = (self.sample_rate as f32 / 250.0) as u32;
        // println!("evs {}", evs.len());

        let ticks_per_sec = (self.pxtone.beat_clock() as f32 * self.pxtone.beat_tempo()) / 60.0;

        let mut skip = 0;
        // only check events every 100 samples, minor performance boost
        for ch in buffer.chunks_mut(100) {
            profiling::scope!("chunk");
            let clock_secs = self.smp as f32 / self.sample_rate as f32;
            let clock_ticks = clock_secs * ticks_per_sec;
            {
                profiling::scope!("events");
                for (i, e) in evs.iter().enumerate().skip(skip) {
                    // if e.unit_no() != 0 { continue; }

                    let e_clock = e.clock() as f32;

                    if e_clock < self.last_clock {
                        continue;
                    }
                    if e_clock > clock_ticks {
                        skip = i - 1;
                        break;
                    }

                    match e.kind() {
                        GenericEventKind::On(on) => {
                            let data = self.unit_data.entry(e.unit_no()).or_default();
                            data.key_now = data.key_start + data.key_margin;
                            data.key_start = data.key_now;
                            data.key_margin = 0;
                            data.on = Some(UnitOnData {
                                start: on.clock(),
                                length: on.length(),
                                cycle: 0.0,
                            });
                        },
                        GenericEventKind::Key(key) => {
                            let key_v = key.key();

                            let data = self.unit_data.entry(e.unit_no()).or_default();

                            data.key_start = data.key_now;
                            data.key_margin = key_v - data.key_start;
                            data.porta_start = e.clock();
                        },
                        GenericEventKind::Velocity(vel) => {
                            self.unit_data.entry(e.unit_no()).or_default().velocity =
                                vel.velocity();
                        },
                        GenericEventKind::Volume(vol) => {
                            self.unit_data.entry(e.unit_no()).or_default().volume = vol.volume();
                        },
                        GenericEventKind::VoiceNo(voice) => {
                            // TODO: I think voice no is supposed to reset porta
                            self.unit_data.entry(e.unit_no()).or_default().woice = voice.voice_no();
                        },
                        GenericEventKind::Tuning(tuning) => {
                            self.unit_data.entry(e.unit_no()).or_default().tuning = tuning.tuning();
                        },
                        GenericEventKind::Porta(porta) => {
                            self.unit_data.entry(e.unit_no()).or_default().porta = porta.porta();
                        },
                        GenericEventKind::PanVolume(pan_volume) => {
                            self.unit_data.entry(e.unit_no()).or_default().pan_volume =
                                pan_volume.pan_volume();
                        },
                        _ => {},
                    }
                }
            }

            {
                profiling::scope!("sample chunk");
                for bsmp in ch.chunks_mut(self.channels as _) {
                    profiling::scope!("one sample");
                    let clock_secs = self.smp as f32 / self.sample_rate as f32;
                    let delta = clock_secs - self.last_sample_clock_secs;
                    let clock_ticks = clock_secs * ticks_per_sec;

                    // println!("skip {skip}");
                    let mut v: Box<[f32]> = (0..bsmp.len()).map(|_| 0.0).collect();

                    #[allow(clippy::for_kv_map)]
                    for (_unit, data) in &mut self.unit_data {
                        if let Some(on) = &mut data.on {

                            // let on_ticks = clock_ticks - on.start as f32;
                            // let on_secs = on_ticks / ticks_per_sec;

                            // porta
                            if data.porta > 0 && data.key_margin != 0 {
                                let thru =
                                    (clock_ticks - data.porta_start as f32) / data.porta as f32;
                                let thru = thru.clamp(0.0, 1.0);
                                data.key_now =
                                    (data.key_start as f32 + data.key_margin as f32 * thru) as _;
                            } else {
                                data.key_now = data.key_start + data.key_margin;
                            }

                            // TODO: make this not witchcraft
                            // 16.3515 is C0 in Hz
                            // 13056 is the "note unit" for C0
                            // 256 "note units" per real semitone
                            // 1.05946^x == 2^(x/12)
                            // 1.05946 == 2^(1/12)
                            #[allow(clippy::excessive_precision)]
                            let key_freq = 16.3515
                                * (1.0594630943592953_f32)
                                    .powf((data.key_now as f32 - 13056.0) / 256.0);

                            on.cycle += (delta * key_freq * *data.tuning) as f64;
                            // on.cycle = (on_secs * key_freq * *data.tuning) as f64;
                            let cycle = on.cycle as f32;

                            // println!("{delta} {key_freq} {} {} {}", *data.tuning, delta * key_freq * *data.tuning, data.cycle);

                            let woice = &self.pxtone.woices.get(data.woice as usize);

                            if let Some(woice) = woice {
                                let pan_volumes = if self.channels == 2 {
                                    [
                                        (1.0 - *data.pan_volume).clamp(0.0, 1.0),
                                        (*data.pan_volume + 1.0).clamp(0.0, 1.0),
                                    ]
                                } else {
                                    [1.0, 1.0]
                                };

                                #[allow(clippy::single_match)]
                                match woice.woice_type() {
                                    WoiceType::PCM(pcm) => {

                                        if clock_ticks > (on.start + on.length) as f32 {
                                            data.on = None;
                                            continue;
                                        }

                                        for (ch, v) in v.iter_mut().enumerate() {
                                            let mut val = pcm.voice.sample(cycle, ch as _);

                                            if pcm.voice.flag_smooth
                                                && cycle * 44100.0 < smooth_smps as f32
                                            {
                                                val *= (cycle * 44100.0) / smooth_smps as f32;
                                            }

                                            *v += val
                                                * *data.volume
                                                * *data.velocity
                                                * pan_volumes[ch]
                                                * i16::MAX as f32;
                                        }
                                    },
                                    WoiceType::OGGV(oggv) => {

                                        if clock_ticks > (on.start + on.length) as f32 {
                                            data.on = None;
                                            continue;
                                        }

                                        for (ch, v) in v.iter_mut().enumerate() {
                                            let mut val = oggv.voice.sample(cycle, ch as _);

                                            if oggv.voice.flag_smooth
                                                && cycle * 44100.0 < smooth_smps as f32
                                            {
                                                val *= (cycle * 44100.0) / smooth_smps as f32;
                                            }

                                            *v += val
                                                * *data.volume
                                                * *data.velocity
                                                * pan_volumes[ch]
                                                * i16::MAX as f32;
                                        }
                                    },
                                    WoiceType::PTV(ptv) => {

                                        let max_env_release_samples = ptv.voices.iter().map(|v| {
                                            if v.envelope.tail_num > 0 {
                                                v.envelope.points[v.envelope.head_num as usize].x * self.sample_rate / v.envelope.fps
                                            } else {
                                                0
                                            }
                                        }).max().unwrap_or(0);

                                        // samples / samples/sec * ticks/seconds = ticks
                                        let max_env_release_ticks = (max_env_release_samples as f32 / self.sample_rate as f32 * ticks_per_sec) as u32;

                                        if clock_ticks > (on.start + on.length + max_env_release_ticks) as f32 {
                                            data.on = None;
                                            continue;
                                        }

                                        for voice in &ptv.voices {

                                            let env_release_samples = if voice.envelope.tail_num > 0 {
                                                voice.envelope.points[voice.envelope.head_num as usize].x * self.sample_rate / voice.envelope.fps
                                            } else {
                                                0
                                            };

                                            // samples / samples/sec * ticks/seconds = ticks
                                            let env_release_ticks = env_release_samples as f32 / self.sample_rate as f32 * ticks_per_sec;

                                            for (ch, v) in v.iter_mut().enumerate() {
                                                let mut val = voice.sample(cycle, ch as _);

                                                let flag_smooth = true;
                                                if flag_smooth
                                                    && cycle * 44100.0 < smooth_smps as f32
                                                {
                                                    val *= (cycle * 44100.0) / smooth_smps as f32;
                                                }

                                                if clock_ticks > (on.start + on.length) as f32 {
                                                    val *= (1.0 - (clock_ticks - (on.start + on.length) as f32) / env_release_ticks).clamp(0.0, 1.0);
                                                }

                                                *v += val
                                                    * *data.volume
                                                    * *data.velocity
                                                    * pan_volumes[ch]
                                                    * i16::MAX as f32;
                                            }
                                        }
                                    },
                                    _ => {},
                                };
                            }
                        }
                    }

                    for (ch, v) in v.iter().enumerate() {
                        bsmp[ch] = (v / 2.0 * self.master_volume)
                            .clamp(i16::MIN as f32, i16::MAX as f32)
                            as _;
                    }
                    self.smp += 1;
                    self.last_sample_clock_secs = clock_secs;
                }
            }

            self.last_clock = clock_ticks;
        }

        profiling::finish_frame!();
        // println!("done");
        Ok(())
    }

    fn is_done_sampling(&self) -> bool {
        todo!()
    }

    fn now_clock(&self) -> u32 {
        self.last_clock as u32
    }

    fn end_clock(&self) -> u32 {
        todo!()
    }

    fn set_unit_mute_enabled(&mut self, _unit_mute: bool) -> Result<(), RPxToneMooError> {
        todo!()
    }

    fn set_loop(&mut self, _should_loop: bool) -> Result<(), RPxToneMooError> {
        todo!()
    }

    fn set_fade(
        &mut self,
        _fade: Option<crate::interface::moo::Fade>,
        _duration: std::time::Duration,
    ) -> Result<(), RPxToneMooError> {
        todo!()
    }

    fn sampling_offset(&self) -> u32 {
        todo!()
    }

    fn sampling_end(&self) -> u32 {
        todo!()
    }

    #[allow(clippy::cast_precision_loss)]
    fn total_samples(&self) -> u32 {
        let total_beats = (self.beat_num() * self.num_measures()) as u32;
        println!("{} {} {}", self.sample_rate, total_beats, self.beat_tempo());
        (self.sample_rate as f32 * 60.0 * total_beats as f32 / self.beat_tempo()) as u32
    }

    fn set_master_volume(&mut self, volume: f32) -> Result<(), RPxToneMooError> {
        self.master_volume = volume;

        Ok(())
    }
}
