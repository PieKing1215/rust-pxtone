use std::{marker::Send, path::Path, time::Duration};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use pxtone::{
    interface::{
        io::PxToneServiceIO,
        moo::{AsMoo, Moo},
        service::PxTone,
    },
    rust_impl::service::RPxTone,
    util::BoxOrMut,
};

fn main() {
    // init pxtone
    let bytes = std::fs::read(Path::new(
        &std::env::args()
            .nth(1)
            .unwrap_or_else(|| "examples/sample.ptcop".into()),
    ))
    .unwrap();
    let mut pxtone = RPxTone::new();

    do_stuff(&bytes, &mut pxtone).unwrap();
}

fn do_stuff<'a, PXTN: PxTone + PxToneServiceIO + AsMoo>(
    bytes: &[u8],
    pxtone: &'a mut PXTN,
) -> Result<(), <PXTN as PxToneServiceIO>::Error>
where
    <PXTN as AsMoo>::M<'a>: Send + Sync,
{
    pxtone.read_bytes(bytes).expect("read_bytes failed");

    // moo
    play(pxtone.as_moo()).unwrap();

    Ok(())
}

fn play<'a, M: Moo<'a> + Send + Sync>(mut moo: BoxOrMut<M>) -> Result<(), M::Error> {
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

    moo.set_audio_format(config.channels() as u8, config.sample_rate().0)
        .expect("set_audio_format failed");

    // prepare to moo
    moo.prepare_sample().expect("prepare_sample failed");
    let total_samples = moo.total_samples();
    println!("pxtone.get_total_samples() = {}", total_samples);
    let mut sn = 10000;
    sn = sn - (sn % 4);
    println!("sn = {}", sn);
    let mut mem: Vec<i16> = vec![0; sn as usize * 2];

    moo.sample(&mut mem).expect("sample failed");

    println!("Mooed to buffer, playing back...");

    // cpal requires borrowed data to be 'static for some reason, so we have to do dumb stuff with channels and threads to access the PxTone
    // Another way would be moving the PxTone into cpal's closure but then you couldn't use it elsewhere...
    // cpal seems to be in no rush to changing this: https://github.com/RustAudio/cpal/issues/434

    let (sender, reciever) = std::sync::mpsc::channel::<usize>();
    let (sender2, reciever2) = std::sync::mpsc::channel::<Vec<i16>>();

    let (exit_tx, exit_rx) = std::sync::mpsc::channel::<()>();

    std::thread::scope(|s| {
        let handle = s.spawn(move || {
            while exit_rx.try_recv().is_err() {
                if let Ok(s) = reciever.try_recv() {
                    let mut mem: Vec<i16> = vec![0; s];
                    let start = std::time::Instant::now();
                    moo.sample(&mut mem).unwrap();
                    println!(
                        "{:.1}ms",
                        std::time::Instant::now().duration_since(start).as_micros() as f32 / 1000.0
                    );
                    sender2.send(mem).unwrap();
                } else {
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
        });

        // play back audio
        let mut sample_i = 0;
        // let mut next_value = move ;
        let data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // let mut mutex = moo;
            write_data(data, 2, &mut || {
                if sample_i >= mem.len() {
                    sender.send(mem.len()).unwrap();
                    let dat = reciever2.recv().unwrap();
                    mem = dat;
                    sample_i -= mem.len();
                }

                let v = mem[sample_i];
                sample_i += 1;
                v as f32 / i16::MAX as f32
            })
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let sample_rate = config.sample_rate().0;
        let stream = device
            .build_output_stream(&config.into(), data_fn, err_fn)
            .expect("Failed to start audio stream");
        stream.play().expect("Failed to play audio");

        let play_time = total_samples as f64 / sample_rate as f64;
        println!("Sleeping for {:.2}s", play_time);
        std::thread::sleep(std::time::Duration::from_secs_f64(play_time));
        println!("Done!");

        exit_tx.send(()).unwrap();
        handle.join().unwrap();
    });

    Ok(())
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
