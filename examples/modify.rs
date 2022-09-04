use std::path::Path;

use pxtone::{
    interface::{
        event::{Event, EventKind, EventListMut},
        io::PxToneServiceIO,
        service::PxTone,
        unit::Unit,
        woice::{PTNOscillator, PTVWaveType, Woice, WoiceType, WoicesMut},
    },
    og_impl::service::PxToneService,
};

fn main() {
    // load ptcop file
    let bytes = include_bytes!("sample.ptcop");
    let mut pxtone = PxToneService::read_bytes(bytes).expect("read_bytes failed");

    do_stuff(&mut pxtone).unwrap();

    println!("Wrote to out.ptcop!");
}

fn do_stuff<PXTN: PxTone + PxToneServiceIO>(pxtone: &mut PXTN) -> Result<(), PXTN::Error> {
    println!("Editing \"{}\"", pxtone.name());

    // change project name/comment
    pxtone.set_name(format!("{}!", pxtone.name())).unwrap();
    pxtone
        .set_comment("this is\r\nthe new comment".into())
        .unwrap();

    // change project tempo
    pxtone.set_beat_tempo(pxtone.beat_tempo() as f32 * 0.75);

    // rename the first unit
    pxtone.units_mut()[0]
        .set_name("supreme unit".into())
        .unwrap();

    // iterate through woices and print out a bunch of details
    for (i, mut woice) in pxtone.woices_mut().iter_mut().enumerate() {
        println!("Woice #{i} \"{}\"", woice.name());

        // edit woice name
        woice.set_name(format!("woice #{i}")).unwrap();

        // print details
        match woice.woice_type() {
            WoiceType::PCM(pcm) => {
                let v = pcm.voice();
                println!(
                    "-PCM chs={} sps={} bps={}",
                    v.channels(),
                    v.samples_per_second(),
                    v.bits_per_sample()
                );
            },
            WoiceType::PTV(ptv) => {
                println!("-PTV with {} voice(s):", ptv.voices().len());
                for (i, v) in ptv.voices().iter().enumerate() {
                    match v.wave() {
                        PTVWaveType::Coordinate(wave) => {
                            println!(
                                "--#{i} (coordinate) reso={} num_points={}",
                                wave.resolution(),
                                wave.points().len()
                            );
                        },
                        PTVWaveType::Overtone(wave) => {
                            println!("--#{i} (overtone) num_tones={}", wave.tones().len());
                        },
                    }
                }
            },
            WoiceType::PTN(ptn) => {
                let v = ptn.voice();
                println!(
                    "-PTN smp_num={} secs={} with {} unit(s)",
                    v.ptn_sample_num(),
                    v.ptn_sample_num() as f32 / 44100.0,
                    v.units().len()
                );
                for (i, u) in v.units().iter().enumerate() {
                    fn osc_to_string(osc: &dyn PTNOscillator) -> String {
                        format!(
                            "shape={:?} freq={:.1} vol={:.1} ofs={:.1} rev={}",
                            osc.shape(),
                            osc.frequency(),
                            osc.volume(),
                            osc.offset(),
                            osc.reverse()
                        )
                    }

                    println!(
                        "--#{i} enabled={} num_envelope_points={} pan={}",
                        u.enabled(),
                        u.envelope().len(),
                        u.pan()
                    );
                    println!("---main={{{}}}", osc_to_string(u.osc_main()));
                    println!("---freq={{{}}}", osc_to_string(u.osc_frequency()));
                    println!("---volu={{{}}}", osc_to_string(u.osc_volume()));
                }
            },
            WoiceType::OGGV(ogg) => {
                let v = ogg.voice();
                println!(
                    "-OGGV ogg_chs={} ogg_sps={} ogg_smp_num={}",
                    v.ogg_channels(),
                    v.ogg_samples_per_second(),
                    v.ogg_sample_num()
                );
            },
            _ => {},
        }
        println!("");
    }

    // edit some events
    for event in pxtone.event_list_mut().iter_mut() {
        // time warp all events
        event.set_clock(event.clock() + ((event.clock() as f32 / 400.0).sin() * 100.0) as i32);

        // flip volume pan
        match event.kind() {
            EventKind::PanVolume => {
                event.set_value(128 - event.value());
            },
            _ => {},
        }
    }

    // write file
    pxtone.write_file(Path::new("out.ptcop"))?;

    Ok(())
}
