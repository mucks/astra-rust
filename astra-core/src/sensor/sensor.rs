use super::*;
use crate::sensor::Frame;
use model::{Body, ColorData, ColorMeta, FrameType, StreamType};
use model::{Error, Result};
use std::collections::HashMap;
use wrapper::*;

pub struct Sensor {
    stream_set: StreamSet,
    reader: Reader,
    streams: HashMap<StreamType, Stream>,
    body_frame_index: i32,
    color_frame_index: i32,
}

impl Sensor {
    pub fn new() -> Result<Sensor> {
        init();
        let stream_set = get_stream_set()?;
        let reader = get_reader(stream_set)?;

        Ok(Sensor {
            stream_set: stream_set,
            reader: reader,
            streams: HashMap::new(),
            body_frame_index: -1,
            color_frame_index: -1,
        })
    }
    pub fn update(&self) {
        update();
    }

    pub fn get_bodies(&mut self) -> Result<Vec<Body>> {
        if let Ok(frame) = frame::Frame::new(self.reader) {
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
    }

    pub fn get_color_bytes(&mut self, frame: Frame) -> Result<Vec<u8>> {
        if let Some(meta) = frame.color_meta {
            Ok(get_color_bytes(
                frame.get_color_frame()?,
                meta.byte_length as u32,
            ))
        } else {
            Err(Error::NoUpdateCallError)
        }
    }

    pub fn update_color(&mut self) -> Result<Frame> {
        self.update();

        let mut frame = frame::Frame::new(self.reader)?;
        let color_frame = frame.get_color_frame()?;
        let frame_index = get_color_frame_index(color_frame);
        if self.color_frame_index != frame_index {
            self.color_frame_index = frame_index;
            let (width, height) = get_color_frame_dimensions(color_frame);
            let byte_length = get_color_frame_byte_length(color_frame);
            frame.color_meta = Some(ColorMeta {
                frame_index,
                width,
                height,
                byte_length,
            });
            Ok(frame)
        } else {
            Err(Error::NoNewFrameError)
        }
    }

    pub fn get_color_ptr(&mut self, frame: Frame, ptr: *mut u8) -> Result<()> {
        get_color_byte_array(frame.get_color_frame()?, ptr)
    }

    pub fn start_body_stream(&mut self) -> Result<()> {
        self.start_stream(StreamType::Body)
    }

    pub fn start_color_stream(&mut self) -> Result<()> {
        self.start_stream(StreamType::Color)
    }

    pub fn start_masked_color_stream(&mut self) -> Result<()> {
        self.start_stream(StreamType::MaskedColor)
    }

    pub fn stop_streams(&mut self) {
        for (_, stream) in &self.streams {
            stop_stream(*stream);
        }
    }

    pub fn stop_all(&mut self) {
        self.stop_streams();
        close_reader(&mut self.reader);
        close_stream_set(&mut self.stream_set);
        terminate();
    }

    fn start_stream(&mut self, st: StreamType) -> Result<()> {
        if self.streams.contains_key(&st) {
            Err(Error::StreamAlreadyStartedError(st))
        } else {
            self.streams.insert(st, start_stream(self.reader, st)?);
            Ok(())
        }
    }
}

impl Drop for Sensor {
    fn drop(&mut self) {
        self.stop_all();
    }
}
