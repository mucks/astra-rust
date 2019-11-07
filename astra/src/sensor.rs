use super::*;
use crate::err::Error;
use crate::frame::Frame;
use std::collections::HashMap;

pub struct Sensor {
    stream_set: Option<StreamSet>,
    reader: Option<Reader>,
    streams: HashMap<StreamType, Stream>,
    frame_indexes: HashMap<StreamType, Stream>,
    index: i32,
}

impl Sensor {
    pub fn new() -> Sensor {
        Sensor {
            stream_set: None,
            reader: None,
            streams: HashMap::new(),
            frame_indexes: HashMap::new(),
            index: -1,
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        if self.stream_set.is_none() && self.reader.is_none() {
            init();
            let stream_set = get_sensor();
            let reader = get_reader(stream_set);
            self.stream_set = Some(stream_set);
            self.reader = Some(reader);
            Ok(())
        } else {
            Err(Error::SensorAlreadyStartedError)
        }
    }
    pub fn get_color_bytes(&mut self) -> Result<Vec<u8>, Error> {
        if let Some(reader) = self.reader {
            if let Ok(frame) = frame::Frame::new(reader) {
                let color_frame = frame.get_color_frame();
                let index = get_color_frame_index(color_frame);
                if self.index != index {
                    self.index = index;

                    let byte_length = get_color_frame_byte_length(color_frame);
                    Ok(get_color_bytes(color_frame, byte_length as u32))
                } else {
                    Err(Error::NoNewFrameError)
                }
            } else {
                Err(Error::CouldNotGetFrameError)
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

        if let Some(stream_set) = &mut self.stream_set {
            close_stream_set(stream_set);
        }
        if let Some(reader) = &mut self.reader {
            close_reader(reader);
        }
        terminate();
    }
}

impl Drop for Sensor {
    fn drop(&mut self) {
        self.stop_all();
    }
}
