use std::fmt;

#[derive(Debug)]
pub struct InvalidText;

impl fmt::Display for InvalidText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Text")
    }
}

impl std::error::Error for InvalidText {}

/// Base trait for the overall state of pxtone
pub trait PxTone {
    fn beat_num(&self) -> i32;
    fn set_beat_num(&mut self, beat_num: i32);

    fn beat_tempo(&self) -> f32;
    fn set_beat_tempo(&mut self, beat_tempo: f32);

    fn beat_clock(&self) -> i32;
    fn set_beat_clock(&mut self, beat_clock: i32);

    fn num_measures(&self) -> i32;
    fn set_num_measures(&mut self, num_measures: i32);

    fn repeat_measure(&self) -> i32;
    fn set_repeat_measure(&mut self, repeat_measure: i32);

    fn last_measure(&self) -> i32;
    fn set_last_measure(&mut self, last_measure: i32);

    fn name(&self) -> String;
    fn set_name(&mut self, name: String) -> Result<(), InvalidText>;

    fn comment(&self) -> String;
    fn set_comment(&mut self, comment: String) -> Result<(), InvalidText>;
}
