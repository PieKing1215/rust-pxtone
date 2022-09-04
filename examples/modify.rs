use std::{path::Path};

use pxtone::{interface::{event::{Event, EventKind, EventListMut}, unit::Unit, woice::{Woice, WoicesMut}, io::PxToneServiceIO, service::PxTone}, og_impl::{service::PxToneService}};

fn main() {
    // load ptcop file
    let bytes = include_bytes!("sample.ptcop");
    let mut pxtone = PxToneService::read_bytes(bytes).expect("read_bytes failed");

    do_stuff(&mut pxtone).unwrap();

    print!("Wrote to out.ptcop!");
}

fn do_stuff<PXTN: PxTone + PxToneServiceIO>(pxtone: &mut PXTN) -> Result<(), PXTN::Error> {
    
    println!("Editing \"{}\"", pxtone.name());

    // change project name/comment
    pxtone.set_name(format!("{}!", pxtone.name())).unwrap();
    pxtone.set_comment("this is\r\nthe new comment".into()).unwrap();

    // change project tempo
    pxtone.set_beat_tempo(pxtone.beat_tempo() as f32 * 0.75);

    // rename the first unit
    pxtone.units_mut()[0].set_name("supreme unit".into()).unwrap();

    // edit woices
    for (i, mut woice) in pxtone.woices_mut().iter_mut().enumerate() {
        woice.set_name(format!("woice #{i}")).unwrap();
    }

    // edit some events
    for event in pxtone.event_list_mut().iter_mut() {
        // time warp all events
        event.set_clock(event.clock() + ((event.clock() as f32 / 400.0).sin() * 100.0) as i32);

        // flip volume pan
        match event.kind() {
            EventKind::PanVolume => {
                event.set_value(128 - event.value());
            }
            _ => {},
        }
    }

    // write file
    pxtone.write_file(Path::new("out.ptcop"))?;

    Ok(())
}