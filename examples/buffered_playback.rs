use std::path::Path;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use pxtone::{
    interface::{io::PxToneServiceIO, moo::Moo},
    og_impl::service::PxToneService,
};

fn main() {
    // set up audio device
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find output device");

    println!(
        "Output device: {}",
        device.name().unwrap_or_else(|_| "unknown".to_string())
    );

    let config = device.default_output_config().unwrap();

    // init pxtone
    let bytes = std::fs::read(Path::new(
        &std::env::args()
            .nth(1)
            .unwrap_or_else(|| "examples/sample.ptcop".into()),
    ))
    .unwrap();
    let mut pxtone = PxToneService::new().expect("PxToneService::new failed");
    pxtone.read_bytes(&bytes).expect("read_bytes failed");
    pxtone
        .set_audio_format(config.channels() as u8, config.sample_rate().0)
        .expect("set_audio_format failed");

    // print some info
    // println!("serv.moo_get_end_clock() = {}", serv.moo_get_end_clock());
    // println!("serv.Unit_Num() = {}", serv.Unit_Num());

    // prepare to moo
    pxtone.prepare_sample().expect("prepare_sample failed");
    let total_samples = pxtone.total_samples();
    println!("pxtone.get_total_samples() = {}", total_samples);
    let mut sn = 10000;
    sn = sn - (sn % 4);
    println!("sn = {}", sn);
    let mut mem: Vec<i16> = vec![0; sn as usize * 2];

    // moo
    pxtone.sample(&mut mem).expect("sample failed");

    println!("Mooed to buffer, playing back...");

    // play back audio
    let mut sample_i = 0;
    let mut next_value = move || {
        if sample_i >= mem.len() {
            let start = std::time::Instant::now();
            pxtone.sample(&mut mem).expect("sample failed");
            println!(
                "{:.1}ms",
                std::time::Instant::now().duration_since(start).as_micros() as f32 / 1000.0
            );
            sample_i -= mem.len();
        }

        let v = mem[sample_i];
        sample_i += 1;
        v as f32 / i16::MAX as f32
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let sample_rate = config.sample_rate().0;
    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, 2, &mut next_value)
            },
            err_fn,
        )
        .expect("Failed to start audio stream");
    stream.play().expect("Failed to play audio");

    let play_time = total_samples as f64 / sample_rate as f64;
    println!("Sleeping for {:.2}s", play_time);
    std::thread::sleep(std::time::Duration::from_secs_f64(play_time));
    println!("Done!");
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    let n = output.chunks_mut(channels);
    for frame in n {
        for sample in frame.iter_mut() {
            let value: T = cpal::Sample::from::<f32>(&next_sample());
            *sample = value;
        }
    }
}
