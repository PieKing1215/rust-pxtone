use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::interface::{
    event::{EventKind, EventListMut, HasEventList},
    event_impl::EventImpl,
    io::PxToneServiceIO,
    service::PxTone,
};

use super::service::RPxTone;

pub struct RPxToneIO {}

#[derive(Debug)]
pub enum RPxToneIOError {
    IncorrectHeader(String),
    BlockNotFound(String),
}

impl PxToneServiceIO for RPxTone {
    type Error = RPxToneIOError;

    fn read_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut c = Cursor::new(bytes);

        let mut name_buf = [0_u8; 16];
        c.read_exact(&mut name_buf).unwrap();

        if name_buf != *b"PTCOLLAGE-071119" {
            return Err(RPxToneIOError::IncorrectHeader(
                String::from_utf8(name_buf.into()).unwrap(),
            ));
        }

        // MasterV5
        seek_block(&mut c, "MasterV5")?;
        assert_eq!(c.read_u32::<LittleEndian>().unwrap(), 15);
        let beat_clock = c.read_i16::<LittleEndian>().unwrap();
        let beat_num = c.read_i8().unwrap();
        let beat_tempo = c.read_f32::<LittleEndian>().unwrap();
        let clock_repeat = c.read_i32::<LittleEndian>().unwrap();
        let clock_last = c.read_i32::<LittleEndian>().unwrap();

        let repeat_measure = clock_repeat / (beat_num as i32 * beat_clock as i32);
        let last_measure = clock_last / (beat_num as i32 * beat_clock as i32);

        let mut num_measures = 1;

        if repeat_measure >= num_measures {
            num_measures = repeat_measure + 1;
        }

        if last_measure > num_measures {
            num_measures = last_measure;
        }

        self.set_beat_clock(beat_clock as _);
        self.set_beat_num(beat_num as _);
        self.set_beat_tempo(beat_tempo);
        self.set_repeat_measure(repeat_measure);
        self.set_last_measure(last_measure);

        seek_block(&mut c, "Event V5")?;
        let _block_size = c.read_u32::<LittleEndian>().unwrap();

        let num_events = c.read_u32::<LittleEndian>().unwrap();
        println!("num_events = {num_events}");

        let mut abs_position = 0;
        let mut last = 0;

        for _ in 0..num_events {
            let pos: u32 = v_r(&mut c).unwrap();
            let unit_no = c.read_u8().unwrap();
            let et = c.read_u8().unwrap();
            // println!("{et}");
            let event_kind: EventKind = et.into();
            let event_value: u32 = v_r(&mut c).unwrap();

            if event_kind == EventKind::Null {
                // TODO: this should probably return an Err
                eprintln!("Invalid event!");
                continue;
            }

            abs_position += pos;

            last = last.max(abs_position);

            self.event_list_mut()
                .add(&EventImpl::from_raw(abs_position, unit_no, event_kind, event_value).unwrap())
                .unwrap();
        }

        num_measures = num_measures
            .max((last as f64 / self.beat_num() as f64 / self.beat_clock() as f64).ceil() as _);
        self.set_num_measures(num_measures);

        Ok(())
    }

    fn write_file(&mut self, path: impl Into<std::path::PathBuf>) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }
}

fn seek_block(c: &mut Cursor<&[u8]>, block: &str) -> Result<(), RPxToneIOError> {
    let p = c.position();
    c.set_position(0);
    let dat = c.get_ref();
    let search = block.as_bytes();
    if let Some(pos) = dat.windows(search.len()).position(|w| w == search) {
        c.set_position(pos as u64 + 8);
        Ok(())
    } else {
        c.set_position(p);
        Err(RPxToneIOError::BlockNotFound(block.into()))
    }
}

fn v_r(c: &mut Cursor<&[u8]>) -> Result<u32, std::io::Error> {
    let mut a = [0_u8; 5];
    let mut b = [0_u8; 5];
    let mut i = 0;
    while i < 5 {
        a[i] = c.read_u8()?;
        if a[i] & 0x80 == 0 {
            break;
        }
        i += 1;
    }

    match i {
        0 => {
            b[0] = a[0] & 0x7F;
        },
        1 => {
            b[0] = (a[0] & 0x7F) | (a[1] << 7);
            b[1] = (a[1] & 0x7F) >> 1;
        },
        2 => {
            b[0] = (a[0] & 0x7F) | (a[1] << 7);
            b[1] = ((a[1] & 0x7F) >> 1) | (a[2] << 6);
            b[2] = (a[2] & 0x7F) >> 2;
        },
        3 => {
            b[0] = (a[0] & 0x7F) | (a[1] << 7);
            b[1] = ((a[1] & 0x7F) >> 1) | (a[2] << 6);
            b[2] = ((a[2] & 0x7F) >> 2) | (a[3] << 5);
            b[3] = (a[3] & 0x7F) >> 3;
        },
        4 => {
            b[0] = (a[0] & 0x7F) | (a[1] << 7);
            b[1] = ((a[1] & 0x7F) >> 1) | (a[2] << 6);
            b[2] = ((a[2] & 0x7F) >> 2) | (a[3] << 5);
            b[3] = ((a[3] & 0x7F) >> 3) | (a[4] << 4);
            b[4] = (a[4] & 0x7F) >> 4;
        },
        _ => {
            return Ok(0);
        },
    }

    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
