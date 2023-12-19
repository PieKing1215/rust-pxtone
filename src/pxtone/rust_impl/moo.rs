use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    interface::{
        event::{
            BaseEvent, EventKey, EventOn, EventVelocity, EventVoiceNo, EventVolume, GenericEvent,
            GenericEventKind, EventTuning, TuningValue, EventPorta,
        },
        moo::{AsMoo, Moo, AsMooRef},
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
}

const DEFAULT_KEY: i32 = 24576;

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
            let clock_secs = (self.smp as f32 / self.channels as f32) / self.sample_rate as f32;
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
                            data.on = Some(UnitOnData { start: on.clock(), length: on.length(), cycle: 0.0 });
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
                        _ => {},
                    }
                }
            }

            {
                profiling::scope!("sample chunk");
                for bsmp in ch {
                    profiling::scope!("one sample");
                    let clock_secs =
                        (self.smp as f32 / self.channels as f32) / self.sample_rate as f32;
                    let delta = clock_secs - self.last_sample_clock_secs;
                    let clock_ticks = clock_secs * ticks_per_sec;

                    // println!("skip {skip}");
                    let mut v = 0;

                    #[allow(clippy::for_kv_map)]
                    for (_unit, data) in &mut self.unit_data {
                        if let Some(on) = &mut data.on {
                            if clock_ticks > (on.start + on.length) as f32 {
                                data.on = None;
                                continue;
                            }

                            // let on_ticks = clock_ticks - on.start as f32;
                            // let on_secs = on_ticks / ticks_per_sec;

                            // porta
                            if data.porta > 0 && data.key_margin != 0 {
                                let thru = (clock_ticks - data.porta_start as f32) / data.porta as f32;
                                let thru = thru.clamp(0.0, 1.0);
                                data.key_now = (data.key_start as f32 + data.key_margin as f32 * thru) as _;
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
                            let key_freq =
                                16.3515 * (1.0594630943592953_f32).powf((data.key_now as f32 - 13056.0) / 256.0);

                            on.cycle += (delta * key_freq * *data.tuning) as f64;
                            // on.cycle = (on_secs * key_freq * *data.tuning) as f64;
                            let cycle = on.cycle as f32;

                            // println!("{delta} {key_freq} {} {} {}", *data.tuning, delta * key_freq * *data.tuning, data.cycle);

                            let woice = &self.pxtone.woices.get(data.woice as usize);

                            if let Some(woice) = woice {
                                #[allow(clippy::single_match)]
                                match woice.woice_type() {
                                    WoiceType::PCM(pcm) => {
                                        let mut val = pcm.voice.sample(cycle);

                                        if pcm.voice.flag_smooth
                                            && cycle * 44100.0 < smooth_smps as f32
                                        {
                                            val *= (cycle * 44100.0) / smooth_smps as f32;
                                        }

                                        v += (val * *data.volume * *data.velocity * i16::MAX as f32)
                                            as i16;
                                    },
                                    WoiceType::OGGV(oggv) => {
                                        let mut val = oggv.voice.sample(cycle);

                                        if oggv.voice.flag_smooth
                                            && cycle * 44100.0 < smooth_smps as f32
                                        {
                                            val *= (cycle * 44100.0) / smooth_smps as f32;
                                        }

                                        v += (val * *data.volume * *data.velocity * i16::MAX as f32)
                                            as i16;
                                    }
                                    _ => {},
                                };
                            }
                        }
                    }

                    // println!("{v} {l}");
                    *bsmp = v;
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
        todo!()
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
        (self.sample_rate as f32 * 60.0 * total_beats as f32 / self.beat_tempo() as f32) as u32
    }

    fn set_master_volume(&mut self, _volume: f32) -> Result<(), RPxToneMooError> {
        todo!()
    }
}
