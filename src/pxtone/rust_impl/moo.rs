use std::{
    collections::HashMap,
    f32::consts::PI,
    ops::{Deref, DerefMut},
};

use crate::{
    interface::{
        event::{BaseEvent, EventKey, EventOn, GenericEvent, GenericEventKind},
        moo::{AsMoo, Moo},
        service::PxTone,
    },
    util::BoxOrMut,
};

use super::service::RPxTone;

pub struct RPxToneMoo<'a> {
    pxtone: &'a mut RPxTone,
    channels: u8,
    sample_rate: u32,

    smp: u32,
    last_clock: f32,

    unit_data: HashMap<u8, UnitData>,
}

struct UnitData {
    on: Option<UnitOnData>,
    key: i32,
}

#[allow(clippy::derivable_impls)]
impl Default for UnitData {
    fn default() -> Self {
        Self { on: None, key: 0 }
    }
}

struct UnitOnData {
    start: u32,
    length: u32,
}

impl Deref for RPxToneMoo<'_> {
    type Target = RPxTone;

    fn deref(&self) -> &Self::Target {
        self.pxtone
    }
}

impl DerefMut for RPxToneMoo<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pxtone
    }
}

#[derive(Debug)]
pub enum RPxToneMooError {}

impl AsMoo for RPxTone {
    type M<'a> = RPxToneMoo<'a> where Self: 'a;

    fn as_moo(&mut self) -> BoxOrMut<Self::M<'_>> {
        BoxOrMut::Box(Box::new(RPxToneMoo {
            pxtone: self,
            channels: 2,
            sample_rate: 44100,

            smp: 0,
            last_clock: 0.0,
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

    fn sample(&mut self, buffer: &mut [i16]) -> Result<(), RPxToneMooError> {
        // println!("buf {}", buffer.len());
        let evs = self.pxtone.event_list.events.iter().collect::<Vec<_>>();
        // println!("evs {}", evs.len());

        let ticks_per_sec = (self.pxtone.beat_clock() as f32 * self.pxtone.beat_tempo()) / 60.0;

        let mut skip = 0;
        for ch in buffer.chunks_mut(100) {
            let clock_secs = (self.smp as f32 / self.channels as f32) / self.sample_rate as f32;
            let clock_ticks = clock_secs * ticks_per_sec;

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
                        self.unit_data.entry(e.unit_no()).or_default().on =
                            Some(UnitOnData { start: on.clock(), length: on.length() });
                    },
                    GenericEventKind::Key(key) => {
                        self.unit_data.entry(e.unit_no()).or_default().key = key.key();
                    },
                    _ => {},
                }
            }

            for bsmp in ch {
                let clock_secs = (self.smp as f32 / self.channels as f32) / self.sample_rate as f32;
                let clock_ticks = clock_secs * ticks_per_sec;

                // println!("skip {skip}");
                let mut v = 0;

                for (unit, data) in &mut self.unit_data {
                    if let Some(on) = &mut data.on {
                        if clock_ticks > (on.start + on.length) as f32 {
                            data.on = None;
                            continue;
                        }

                        let note = data.key as f32;

                        // TODO: make this not witchcraft
                        #[warn(clippy::excessive_precision)]
                        let freq =
                            16.3515 * (1.0594630943592953_f32).powf((note - 13056.0) / 256.0);

                        let on_ticks = clock_ticks - on.start as f32;
                        let on_secs = on_ticks / ticks_per_sec;

                        let cycle = on_secs * freq * 2.0;
                        let val = match unit {
                            0 => (cycle * 2.0 * PI).sin(),        // sin
                            1 | 3 => ((cycle % 1.0) - 0.5) * 2.0, // saw
                            2 => {
                                if cycle % 1.0 > 0.5 {
                                    1.0
                                } else {
                                    -1.0
                                }
                            }, // square
                            _ => 0.0,                             // sin
                                                                   // 0 | 1 => {
                                                                   //     if cycle % 1.0 > 0.5 {
                                                                   //         1.0
                                                                   //     } else {
                                                                   //         -1.0
                                                                   //     }
                                                                   // }, // square
                                                                   // 2 | 3 => {
                                                                   //     if cycle % 1.0 > 0.75 {
                                                                   //         1.0
                                                                   //     } else {
                                                                   //         -1.0
                                                                   //     }
                                                                   // }, // 25%
                                                                   // 4 => ((cycle % 1.0) - 0.5) * 2.0, // saw
                                                                   // _ => (cycle * 2.0 * PI).sin(),    // sin
                        };

                        // if *unit == 0 {
                        //     println!("{note} {clock_ticks} {} {cl} {i} {key} {freq} {val} {}", self.smp, clock_ticks / self.sample_rate as f32 * freq);
                        // }
                        v += (val * 256.0) as i16;
                    }
                }

                // println!("{v} {l}");
                *bsmp = v;
                self.smp += 1;
            }

            self.last_clock = clock_ticks;
        }
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

    fn set_unit_mute_enabled(&mut self, unit_mute: bool) -> Result<(), RPxToneMooError> {
        todo!()
    }

    fn set_loop(&mut self, should_loop: bool) -> Result<(), RPxToneMooError> {
        todo!()
    }

    fn set_fade(
        &mut self,
        fade: Option<crate::interface::moo::Fade>,
        duration: std::time::Duration,
    ) -> Result<(), RPxToneMooError> {
        todo!()
    }

    fn sampling_offset(&self) -> u32 {
        todo!()
    }

    fn sampling_end(&self) -> u32 {
        todo!()
    }

    fn total_samples(&self) -> u32 {
        let total_beats = (self.beat_num() * self.num_measures()) as u32;
        println!("{} {} {}", self.sample_rate, total_beats, self.beat_tempo());
        (self.sample_rate as f64 * 60.0 * total_beats as f64 / self.beat_tempo() as f64) as u32
    }

    fn set_master_volume(&mut self, volume: f32) -> Result<(), RPxToneMooError> {
        todo!()
    }
}
