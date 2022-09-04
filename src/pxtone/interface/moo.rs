pub enum Fade {
    In,
    Out,
}

/// Trait that covers everything related to playing/sampling the song
///
/// (This was called "mooing" by Pixel)
pub trait Moo<E> {
    fn set_audio_format(&mut self, channels: u8, sample_rate: u32) -> Result<(), E>;

    // TODO: consider enforcing `prepare_sample` being called before `sample`, eg with a MutexGuard type thing
    fn prepare_sample(&mut self) -> Result<(), E>;
    fn sample(&mut self, buffer: &mut [i16]) -> Result<(), E>;

    fn is_done_sampling(&self) -> bool;
    fn now_clock(&self) -> u32;
    fn end_clock(&self) -> u32;

    fn set_unit_mute_enabled(&mut self, unit_mute: bool) -> Result<(), E>;
    fn set_loop(&mut self, should_loop: bool) -> Result<(), E>;
    fn set_fade(&mut self, fade: Option<Fade>, duration: std::time::Duration) -> Result<(), E>;

    fn sampling_offset(&self) -> u32;
    fn sampling_end(&self) -> u32;
    fn total_samples(&self) -> u32;

    fn set_master_volume(&mut self, volume: f32) -> Result<(), E>;
}
