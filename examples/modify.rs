use std::path::Path;

use pxtone::{
    interface::{
        delay::{Delay, DelayUnit, DelaysMut, HasDelays},
        event::{
            BaseEvent, EventListMut, EventPanVolume, GenericEvent, GenericEventKind, HasEventList,
            PanValue,
        },
        io::PxToneServiceIO,
        overdrive::{HasOverDrives, OverDAmp, OverDCut, OverDrivesMut},
        service::PxTone,
        unit::{HasUnits, Unit, UnitsMut},
        woice::{
            HasWoices, PTNOscillator, PTNUnit, PTVCoordinateWave, PTVOvertoneWave, PTVWaveType,
            SingleVoice, VoiceOGGV, VoicePCM, VoicePTN, VoicePTV, Woice, WoicePTV, WoiceType,
            WoicesMut,
        },
    },
    og_impl::service::PxToneService,
    util::ZeroToOneF32,
};

fn main() {
    // load ptcop file
    let bytes = include_bytes!("sample.ptcop");

    do_stuff::<PxToneService>(bytes).unwrap();
}

// This stuff is in a separate function just to demonstrate how everything is trait based
fn do_stuff<
    PXTN: PxTone + PxToneServiceIO + HasUnits + HasWoices + HasEventList + HasDelays + HasOverDrives,
>(
    bytes: &[u8],
) -> Result<(), PXTN::Error> {
    let mut pxtone = PXTN::read_bytes(bytes).expect("read_bytes failed");

    println!("Editing \"{}\"", pxtone.name());

    // change project name/comment
    pxtone.set_name(format!("{}!", pxtone.name())).unwrap();
    pxtone
        .set_comment("this is\r\nthe new comment".into())
        .unwrap();

    // change project tempo
    pxtone.set_beat_tempo(pxtone.beat_tempo() as f32 * 0.75);

    // rename the first unit
    pxtone
        .units_mut()
        .iter_mut()
        .next()
        .unwrap()
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
                    fn osc_to_string<O: PTNOscillator>(osc: &O) -> String {
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
        println!();
    }

    // edit some events
    for event in pxtone.event_list_mut().iter_mut() {
        // time warp all events
        event.set_clock(event.clock() + ((event.clock() as f32 / 400.0).sin() * 100.0) as u32);

        // flip volume pan
        #[allow(clippy::single_match)]
        match &mut event.kind_mut() {
            GenericEventKind::PanVolume(e) => {
                let e = &mut **e;
                e.set_pan_volume(PanValue::new(-*e.pan_volume()));
            },
            _ => {},
        }
    }

    // add a couple delay effects
    pxtone
        .delays_mut()
        .add(0, DelayUnit::Second(4.0), ZeroToOneF32::new(0.25))
        .unwrap();
    pxtone
        .delays_mut()
        .add(3, DelayUnit::Beat(2.0), ZeroToOneF32::new(0.5))
        .unwrap();

    // edit delay effects
    for mut delay in pxtone.delays_mut().iter_mut() {
        if delay.group() == 3 {
            delay.set_frequency(DelayUnit::Measure(4.0));
        }
    }

    // add an overdrive effect
    pxtone
        .overdrives_mut()
        .add(1, OverDCut::new(0.9), OverDAmp::new(2.0))
        .unwrap();

    // write file
    pxtone.write_file(Path::new("out.ptcop"))?;

    println!("Wrote to out.ptcop!");

    Ok(())
}
