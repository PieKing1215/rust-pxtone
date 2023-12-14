use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
    interface::{
        event::{EventKind, EventListMut, HasEventList},
        event_impl::EventImpl,
        io::PxToneServiceIO,
        service::PxTone,
    },
    rust_impl::woice::{
        RPxToneVoicePCM, RPxToneVoicePCMError, RPxToneWoice, RPxToneWoicePCM, RPxToneWoiceType,
    },
};

use super::{service::RPxTone, woice::{RPxToneWoiceOGGV, RPxToneVoiceOGGV, RPxToneVoiceOGGVError, RPxToneWoicePTV}};

pub struct RPxToneIO {}

#[derive(Debug)]
#[non_exhaustive]
pub enum RPxToneIOError {
    IncorrectHeader(String),
    BlockNotFound(String),
    IncorrectBlockSize {
        block: String,
        expected: u32,
        actual: u32,
    },
    AntiOper,
    InvalidPCMConfig {
        bits_per_sample: u8,
        channels: u8,
    },
    InvalidOGGVConfig {
        samples_per_second: u8,
        channels: u8,
    },
    VorbisError(lewton::VorbisError),
}

impl PxToneServiceIO for RPxTone {
    type Error = RPxToneIOError;

    #[allow(clippy::too_many_lines)]
    fn read_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut c = Cursor::new(bytes);

        let mut name_buf = [0_u8; 16];
        c.read_exact(&mut name_buf).unwrap();

        if name_buf != *b"PTCOLLAGE-071119" {
            return Err(RPxToneIOError::IncorrectHeader(
                String::from_utf8(name_buf.into()).unwrap(),
            ));
        }

        let _exe_ver = c.read_u16::<LittleEndian>().unwrap();
        let _rrr = c.read_u16::<LittleEndian>().unwrap();

        let mut last_eve_pos = 0;

        loop {
            let mut block_name_buf = [0_u8; 8];
            c.read_exact(&mut block_name_buf).unwrap();
            let block_name = String::from_utf8_lossy(&block_name_buf).to_string();
            let block_size = c.read_u32::<LittleEndian>().unwrap();

            match &block_name_buf {
                b"MasterV5" => {
                    if block_size != 15 {
                        return Err(RPxToneIOError::IncorrectBlockSize {
                            block: block_name,
                            expected: 15,
                            actual: block_size,
                        });
                    }

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
                    self.set_num_measures(num_measures);
                },
                b"Event V5" => {
                    let num_events = c.read_u32::<LittleEndian>().unwrap();
                    println!("num_events = {num_events}");

                    let mut abs_position = 0;

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

                        last_eve_pos = last_eve_pos.max(abs_position);

                        self.event_list_mut()
                            .add(
                                &EventImpl::from_raw(
                                    abs_position,
                                    unit_no,
                                    event_kind,
                                    event_value,
                                )
                                .unwrap(),
                            )
                            .unwrap();
                    }
                },
                b"matePCM " => {
                    let _x3x_unit_no = c.read_u16::<LittleEndian>().unwrap();
                    let basic_key = c.read_u16::<LittleEndian>().unwrap();
                    let voice_flags = c.read_u32::<LittleEndian>().unwrap();
                    let channels = c.read_u16::<LittleEndian>().unwrap();
                    let bits_per_sample = c.read_u16::<LittleEndian>().unwrap();
                    let samples_per_second = c.read_u32::<LittleEndian>().unwrap();
                    let tuning = c.read_f32::<LittleEndian>().unwrap();
                    let data_size = c.read_u32::<LittleEndian>().unwrap();

                    assert_eq!(voice_flags & 0xffff_fff8, 0); // only flags 0x1, 0x2, and 0x4 are used

                    // println!("PCM {_x3x_unit_no} {basic_key} {voice_flags} {channels} {bits_per_sample} {samples_per_second} {tuning} {data_size}");

                    let mut data_buf = vec![0_u8; data_size as _];
                    c.read_exact(&mut data_buf).unwrap();

                    self.woices.push(RPxToneWoice {
                        name: String::new(),
                        woice_type: RPxToneWoiceType::PCM(RPxToneWoicePCM {
                            voice: RPxToneVoicePCM::new(
                                basic_key as _,
                                128,
                                64,
                                tuning,
                                channels as _,
                                samples_per_second,
                                bits_per_sample as _,
                                data_buf,
                                voice_flags & 0x1 != 0,
                                voice_flags & 0x2 != 0,
                                voice_flags & 0x4 != 0,
                            )
                            .map_err(|e| match e {
                                RPxToneVoicePCMError::InvalidPCMConfig {
                                    bits_per_sample,
                                    channels,
                                } => RPxToneIOError::InvalidPCMConfig { bits_per_sample, channels },
                            })?,
                        }),
                    });
                },
                b"mateOGGV" => {
                    let _xxx: u16 = c.read_u16::<LittleEndian>().unwrap();
                    let basic_key: u16 = c.read_u16::<LittleEndian>().unwrap();
                    let voice_flags: u32 = c.read_u32::<LittleEndian>().unwrap();
                    let tuning: f32 = c.read_f32::<LittleEndian>().unwrap();

                    let channels = c.read_u32::<LittleEndian>().unwrap();
                    let samples_per_second = c.read_u32::<LittleEndian>().unwrap();
                    let sample_num = c.read_u32::<LittleEndian>().unwrap();
                    let data_size = c.read_u32::<LittleEndian>().unwrap();

                    assert_eq!(voice_flags & 0xffff_fff8, 0); // only flags 0x1, 0x2, and 0x4 are used

                    // println!("PCM {_x3x_unit_no} {basic_key} {voice_flags} {channels} {bits_per_sample} {samples_per_second} {tuning} {data_size}");

                    let mut data_buf = vec![0_u8; data_size as _];
                    c.read_exact(&mut data_buf).unwrap();
                    
                    self.woices.push(RPxToneWoice {
                        name: String::new(),
                        woice_type: RPxToneWoiceType::OGGV(RPxToneWoiceOGGV {
                            voice: RPxToneVoiceOGGV::new(
                                basic_key as _,
                                128,
                                64,
                                tuning,
                                channels as _,
                                samples_per_second,
                                sample_num as _,
                                data_buf,
                                voice_flags & 0x1 != 0,
                                voice_flags & 0x2 != 0,
                                voice_flags & 0x4 != 0,
                            ).map_err(|e| match e {
                                RPxToneVoiceOGGVError::InvalidOGGVConfig {
                                    samples_per_second,
                                    channels,
                                } => RPxToneIOError::InvalidOGGVConfig { samples_per_second, channels },
                                RPxToneVoiceOGGVError::VorbisError(e) => RPxToneIOError::VorbisError(e),
                            })?,
                        }),
                    });
                },
                b"matePTV " => {
                    // let _x3x_unit_no = c.read_u16::<LittleEndian>().unwrap();
                    // let _rrr = c.read_u16::<LittleEndian>().unwrap();
                    // let tuning = c.read_f32::<LittleEndian>().unwrap();
                    // let _size = c.read_u32::<LittleEndian>().unwrap();

                    // println!("PCM {_x3x_unit_no} {_rrr} {tuning} {_size}");

                    self.woices.push(RPxToneWoice {
                        name: String::new(),
                        woice_type: RPxToneWoiceType::PTV(RPxToneWoicePTV {
                            voices: vec![],
                        }),
                    });

                    // TODO: placeholder
                    c.set_position(c.position() + block_size as u64);
                },
                b"matePTN " => {
                    self.woices.push(RPxToneWoice {
                        name: String::new(),
                        woice_type: RPxToneWoiceType::PTV(RPxToneWoicePTV {
                            voices: vec![],
                        }),
                    });

                    // TODO: placeholder
                    c.set_position(c.position() + block_size as u64);
                },
                b"num UNIT" => {
                    if block_size != 4 {
                        return Err(RPxToneIOError::IncorrectBlockSize {
                            block: block_name,
                            expected: 15,
                            actual: block_size,
                        });
                    }

                    let _num_unit = c.read_u32::<LittleEndian>().unwrap();
                },
                b"textNAME" => {
                    let mut name_buf = vec![0_u8; block_size as usize];
                    c.read_exact(&mut name_buf).unwrap();
                    let name = String::from_utf8(name_buf).unwrap();
                    self.set_name(name).unwrap();
                },
                b"textCOMM" => {
                    let mut comment_buf = vec![0_u8; block_size as usize];
                    c.read_exact(&mut comment_buf).unwrap();
                    let comment = String::from_utf8(comment_buf).unwrap();
                    self.set_comment(comment).unwrap();
                },
                b"pxtoneND" => {
                    break;
                },
                b"antiOPER" => {
                    return Err(RPxToneIOError::AntiOper);
                },
                _ => {
                    println!("Unimplemented block: {}", block_name);
                    c.set_position(c.position() + block_size as u64);
                },
            }
        }

        let num_measures = self.num_measures().max(
            (last_eve_pos as f64 / self.beat_num() as f64 / self.beat_clock() as f64).ceil() as _,
        );
        self.set_num_measures(num_measures);

        Ok(())
    }

    fn write_file(&mut self, _path: impl Into<std::path::PathBuf>) -> Result<Vec<u8>, Self::Error> {
        todo!()
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
