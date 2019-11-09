use crate::frame::Frame;
use model::{Body, ColorData, ColorMeta, FrameType, ImageFrameType, StreamType};
use model::{Error, Result};
use std::collections::HashMap;
use wrapper::*;

pub struct Sensor {
    stream_set: StreamSet,
    reader: Reader,
    streams: HashMap<StreamType, Stream>,
    indexes: HashMap<StreamType, i32>,
}

impl Sensor {
    pub fn new() -> Result<Sensor> {
        init()?;
        let stream_set = get_stream_set()?;
        let reader = get_reader(stream_set)?;

        Ok(Sensor {
            stream_set: stream_set,
            reader: reader,
            streams: HashMap::new(),
            indexes: [
                (StreamType::Color, -1),
                (StreamType::Body, -1),
                (StreamType::MaskedColor, -1),
                (StreamType::Depth, -1),
            ]
            .iter()
            .cloned()
            .collect(),
        })
    }
    pub fn update(&self) -> Result<()> {
        update()
    }

    pub fn get_bodies(&mut self) -> Result<Vec<Body>> {
        if let Ok(frame) = Frame::new(self.reader) {
            let body_frame = frame.get_body_frame()?;
            let index = get_body_frame_index(body_frame);
            let body_frame_index = self.indexes.get_mut(&StreamType::Body).unwrap();
            if body_frame_index != &index {
                *body_frame_index = index;
                Ok(get_bodies(body_frame))
            } else {
                Err(Error::NoNewFrameError)
            }
        } else {
            Err(Error::CouldNotGetFrameError(FrameType::Body))
        }
    }

    pub fn get_depth_bytes(&mut self) -> Result<(u32, u32, usize, Vec<u8>)> {
        self.get_img_bytes(StreamType::Depth)
    }

    pub fn get_color_bytes(&mut self) -> Result<(u32, u32, usize, Vec<u8>)> {
        self.get_img_bytes(StreamType::Color)
    }

    pub fn get_masked_color_bytes(&mut self) -> Result<(u32, u32, usize, Vec<u8>)> {
        self.get_img_bytes(StreamType::MaskedColor)
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
    pub fn start_depth_stream(&mut self) -> Result<()> {
        self.start_stream(StreamType::Depth)
    }

    pub fn stop_streams(&mut self) -> Result<()> {
        for (_, stream) in &self.streams {
            stop_stream(*stream)?;
        }
        Ok(())
    }

    pub fn stop_all(&mut self) -> Result<()> {
        self.stop_streams()?;
        close_reader(&mut self.reader)?;
        close_stream_set(&mut self.stream_set)?;
        terminate()
    }

    fn start_stream(&mut self, st: StreamType) -> Result<()> {
        if self.streams.contains_key(&st) {
            Err(Error::StreamAlreadyStartedError(st))
        } else {
            self.streams.insert(st, start_stream(self.reader, st)?);
            Ok(())
        }
    }
    fn get_img_bytes(&mut self, stream_type: StreamType) -> Result<(u32, u32, usize, Vec<u8>)> {
        use self::StreamType::*;
        if let Ok(frame) = Frame::new(self.reader) {
            let img_frame = match stream_type {
                Color => frame.get_color_frame(),
                MaskedColor => frame.get_masked_color_frame(),
                Depth => frame.get_depth_frame(),
                _ => return Err(Error::GetImgFrameError),
            }?;
            let index = get_img_frame_index(img_frame)?;
            let frame_index = self.indexes.get_mut(&stream_type).unwrap();
            if frame_index != &index {
                *frame_index = index;

                match stream_type {
                    Color => get_color_bytes(img_frame),
                    MaskedColor => get_masked_color_bytes(img_frame),
                    Depth => get_depth_bytes(img_frame),
                    _ => Err(Error::GetImgFrameError),
                }
            } else {
                Err(Error::NoNewFrameError)
            }
        } else {
            Err(Error::CouldNotGetFrameError(FrameType::Body))
        }
    }
}

impl Drop for Sensor {
    fn drop(&mut self) {
        self.stop_all();
    }
}
