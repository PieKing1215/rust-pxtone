use crate::interface::service::{InvalidText, PxTone};

use super::{event::RPxToneEventList, woice::RPxToneWoice};

pub struct RPxTone {
    beat_num: i32,
    beat_tempo: f32,
    beat_clock: i32,
    num_measures: i32,
    repeat_measure: i32,
    last_measure: i32,
    name: String,
    comment: String,

    pub(crate) event_list: RPxToneEventList,
    pub(crate) woices: Vec<RPxToneWoice>,
}

impl Default for RPxTone {
    fn default() -> Self {
        Self::new()
    }
}

impl RPxTone {
    #[must_use]
    pub fn new() -> Self {
        Self {
            beat_num: 4,
            beat_tempo: 120.0,
            beat_clock: 480,
            num_measures: 1,
            repeat_measure: 0,
            last_measure: 0,
            name: String::new(),
            comment: String::new(),
            event_list: RPxToneEventList::default(),
            woices: Vec::new(),
        }
    }
}

impl PxTone for RPxTone {
    fn beat_num(&self) -> i32 {
        self.beat_num
    }

    fn set_beat_num(&mut self, beat_num: i32) {
        self.beat_num = beat_num;
    }

    fn beat_tempo(&self) -> f32 {
        self.beat_tempo
    }

    fn set_beat_tempo(&mut self, beat_tempo: f32) {
        self.beat_tempo = beat_tempo;
    }

    fn beat_clock(&self) -> i32 {
        self.beat_clock
    }

    fn set_beat_clock(&mut self, beat_clock: i32) {
        self.beat_clock = beat_clock;
    }

    fn num_measures(&self) -> i32 {
        self.num_measures
    }

    fn set_num_measures(&mut self, num_measures: i32) {
        self.num_measures = num_measures;
    }

    fn repeat_measure(&self) -> i32 {
        self.repeat_measure
    }

    fn set_repeat_measure(&mut self, repeat_measure: i32) {
        self.repeat_measure = repeat_measure;
    }

    fn last_measure(&self) -> i32 {
        self.last_measure
    }

    fn set_last_measure(&mut self, last_measure: i32) {
        self.last_measure = last_measure;
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) -> Result<(), InvalidText> {
        self.name = name;
        Ok(())
    }

    fn comment(&self) -> String {
        self.comment.clone()
    }

    fn set_comment(&mut self, comment: String) -> Result<(), InvalidText> {
        self.comment = comment;
        Ok(())
    }
}
