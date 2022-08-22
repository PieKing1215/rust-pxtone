use std::path::Path;

use pxtone::{og_impl::service::PxToneService, interface::{io::PxToneServiceIO, service::PxTone, event::{EventListMut, Event, EventKind}, unit::Unit}};

fn main() {
    // load ptcop file
    let bytes = include_bytes!("sample.ptcop");
    let mut pxtone = PxToneService::read_bytes(bytes).expect("read_bytes failed");

    println!("Editing \"{}\"", pxtone.name());
    
    // change project name/comment
    pxtone.set_name(format!("{}!", pxtone.name())).unwrap();
    pxtone.set_comment("this is\r\nthe new comment".into()).unwrap();

    // change project tempo
    pxtone.set_beat_tempo(pxtone.beat_tempo() as f32 * 0.75);

    // rename the first unit
    pxtone.units_mut()[0].set_name("supreme unit".into()).unwrap();
    
    // edit some events
    for eve in pxtone.event_list_mut().events_mut() {
        // time warp all events
        eve.set_clock(eve.clock() + ((eve.clock() as f32 / 400.0).sin() * 100.0) as i32);

        // flip volume pan
        match eve.kind() {
            EventKind::PanVolume => {
                eve.set_value(128 - eve.value());
            }
            _ => {},
        }
    }

    // write file
    pxtone.write_file(Path::new("out.ptcop")).unwrap();

    print!("Wrote to out.ptcop!");
}