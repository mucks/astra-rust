use super::*;
use crate::err::Error;
use model::{Body, FrameType, StreamType};
use std::collections::HashMap;
use wrapper::*;

#[derive(Default)]
pub struct Sensor {
    stream_set: Option<StreamSet>,
    reader: Option<Reader>,
    streams: HashMap<StreamType, Stream>,
    color_frame_index: i32,
    body_frame_index: i32,
}

impl Sensor {
    pub fn new() -> Sensor {
        Sensor {
            stream_set: None,
            reader: None,
            streams: HashMap::new(),
            color_frame_index: -1,
            body_frame_index: -1,
        }
    }

    pub fn update(&self) {
        update();
    }

    pub fn start(&mut self) -> Result<(), Error> {
        if self.stream_set.is_none() && self.reader.is_none() {
            init();
            let stream_set = get_stream_set();
            let reader = get_reader(stream_set);
            self.stream_set = Some(stream_set);
            self.reader = Some(reader);
            Ok(())
        } else {
            Err(Error::SensorAlreadyStartedError)
        }
    }

    pub fn get_bodies(&mut self) -> Result<Vec<Body>, Error> {
        if let Some(reader) = self.reader {
            if let Ok(frame) = frame::Frame::new(reader) {
                let body_frame = frame.get_body_frame();
                let index = get_body_frame_index(body_frame);
                if self.body_frame_index != index {
                    self.body_frame_index = index;
                    Ok(get_bodies(body_frame))
                } else {
                    Err(Error::NoNewFrameError)
                }
            } else {
                Err(Error::CouldNotGetFrameError(FrameType::Body))
            }
        } else {
            Err(Error::StreamNotStartedError(StreamType::Body))
        }
    }

    pub fn get_color_bytes(&mut self) -> Result<Vec<u8>, Error> {
        if let Some(reader) = self.reader {
            if let Ok(frame) = frame::Frame::new(reader) {
                let color_frame = frame.get_color_frame();
                let index = get_color_frame_index(color_frame);
                if self.color_frame_index != index {
                    self.color_frame_index = index;

                    let byte_length = get_color_frame_byte_length(color_frame);
                    Ok(get_color_bytes(color_frame, byte_length as u32))
                } else {
                    Err(Error::NoNewFrameError)
                }
            } else {
                Err(Error::CouldNotGetFrameError(FrameType::Color))
            }
        } else {
            Err(Error::StreamNotStartedError(StreamType::Color))
        }
    }

    pub fn start_body_stream(&mut self) -> Result<(), Error> {
        self.start_stream(StreamType::Body)
    }
    pub fn start_color_stream(&mut self) -> Result<(), Error> {
        self.start_stream(StreamType::Color)
    }
    fn start_stream(&mut self, st: StreamType) -> Result<(), Error> {
        if let Some(reader) = self.reader {
            if self.streams.contains_key(&st) {
                Err(Error::StreamAlreadyStartedError(st))
            } else {
                self.streams.insert(st, start_stream(reader, st));
                Ok(())
            }
        } else {
            Err(Error::SensorNotStartedError)
        }
    }

    pub fn stop_streams(&mut self) {
        for (_, stream) in &self.streams {
            stop_stream(*stream);
        }
    }

    pub fn stop_all(&mut self) {
        self.stop_streams();

        if let Some(reader) = &mut self.reader {
            close_reader(reader);
        }
        if let Some(stream_set) = &mut self.stream_set {
            close_stream_set(stream_set);
        }
        terminate();
    }
}

impl Drop for Sensor {
    fn drop(&mut self) {
        self.stop_all();
    }
}
