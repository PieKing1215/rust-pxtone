
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use pxtone::{og_impl::service::PxToneService, interface::{moo::Moo, io::PxToneServiceIO}};

fn main() {
    // set up audio device
    let host = cpal::default_host();
    let device = host.default_output_device().expect("failed to find output device");
    println!("Output device: {}", device.name().unwrap_or("unknown".to_string()));

    let config = device.default_output_config().unwrap();

    // init pxtone
    let bytes = include_bytes!("sample.ptcop");
    let mut pxtone = PxToneService::read_bytes(bytes).expect("read_bytes failed");
    pxtone.set_audio_format(config.channels() as u8, config.sample_rate().0).expect("set_audio_format failed");

    // print some info
    // println!("serv.moo_get_end_clock() = {}", serv.moo_get_end_clock());
    // println!("serv.Unit_Num() = {}", serv.Unit_Num());

    // prepare to moo
    pxtone.prepare_sample().expect("prepare_sample failed");
    println!("pxtone.get_total_samples() = {}", pxtone.total_samples());
    let mut sn = pxtone.total_samples();
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
            return 0.0;
        }

        let v = mem[sample_i];
        sample_i += 1;
        v as f32 / i16::MAX as f32
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let sample_rate = config.sample_rate().0;
    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            write_data(data, 2, &mut next_value)
        },
        err_fn,
    ).expect("Failed to start audio stream");
    stream.play().expect("Failed to play audio");

    println!("Sleeping for {:.2}s", sn as f64 / sample_rate as f64);
    std::thread::sleep(std::time::Duration::from_secs_f64(sn as f64 / sample_rate as f64));
    println!("Done!");
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where T: cpal::Sample,
{
    let n = output.chunks_mut(channels);
    for frame in n {
        for sample in frame.iter_mut() {
            let value: T = cpal::Sample::from::<f32>(&next_sample());
            *sample = value;
        }
    }
}