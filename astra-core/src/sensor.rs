use crate::frame::Frame;
use model::{Body, ColorData, ColorMeta, FrameType, ImageFrameType, StreamType};
use model::{Error, Result};
use std::collections::HashMap;
use wrapper::*;

pub struct Sensor {
    stream_set: Option<StreamSet>,
    reader: Option<Reader>,
    streams: HashMap<StreamType, Stream>,
    indexes: HashMap<StreamType, i32>,
}

impl Sensor {
    pub fn new() -> Sensor {
        Sensor {
            stream_set: None,
            reader: None,
            streams: HashMap::new(),
            indexes: HashMap::new(),
        }
    }
    pub fn init(&mut self) -> Result<()> {
        if self.stream_set.is_none() {
            init()?;
            self.init_indexes();
            self.stream_set = Some(get_stream_set()?);
            self.reader = Some(get_reader(self.stream_set.unwrap())?);
            Ok(())
        } else {
            Err(Error::SensorAlreadyInitializedError)
        }
    }

    fn init_indexes(&mut self) {
        self.indexes = [
            (StreamType::Color, -1),
            (StreamType::Body, -1),
            (StreamType::MaskedColor, -1),
            (StreamType::Depth, -1),
        ]
        .iter()
        .cloned()
        .collect();
    }

    pub fn update(&self) -> Result<Frame> {
        update()?;
        if let Some(reader) = self.reader {
            Frame::new(reader)
        } else {
            Err(Error::SensorNotInitializedError)
        }
    }

    pub fn get_bodies(&mut self, frame: &Frame) -> Result<Vec<Body>> {
        let body_frame = frame.get_body_frame()?;
        let index = get_body_frame_index(body_frame);
        let body_frame_index = self.indexes.get_mut(&StreamType::Body).unwrap();
        if body_frame_index != &index {
            *body_frame_index = index;
            Ok(get_bodies(body_frame))
        } else {
            Err(Error::NoNewFrameError)
        }
    }

    pub fn get_depth_bytes(&mut self, frame: &Frame) -> Result<(u32, u32, usize, Vec<u8>)> {
        self.get_img_bytes(frame, StreamType::Depth)
    }

    pub fn get_color_bytes(&mut self, frame: &Frame) -> Result<(u32, u32, usize, Vec<u8>)> {
        self.get_img_bytes(&frame, StreamType::Color)
    }

    pub fn get_masked_color_bytes(&mut self, frame: &Frame) -> Result<(u32, u32, usize, Vec<u8>)> {
        self.get_img_bytes(&frame, StreamType::MaskedColor)
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
        self.streams = HashMap::new();
        self.init_indexes();
        Ok(())
    }

    pub fn stop_all(&mut self) -> Result<()> {
        println!("closing streams ");
        self.stop_streams()?;

        if let Some(reader) = &mut self.reader {
            println!("closing reader");
            close_reader(reader)?;
            self.reader = None;
        }
        if let Some(stream_set) = &mut self.stream_set {
            println!("closing set");
            close_stream_set(stream_set)?;
            self.stream_set = None;
        }
        if self.reader.is_none() && self.stream_set.is_none() {
            println!("terminate");
            //TODO: find out why it causes corrupted double linked list in godot_astra_plugin
            //terminate()?;
        }
        Ok(())
    }

    fn start_stream(&mut self, st: StreamType) -> Result<()> {
        if self.streams.contains_key(&st) {
            Err(Error::StreamAlreadyStartedError(st))
        } else {
            if let Some(reader) = self.reader {
                self.streams.insert(st, start_stream(reader, st)?);
                Ok(())
            } else {
                Err(Error::SensorNotInitializedError)
            }
        }
    }
    fn get_img_bytes(
        &mut self,
        frame: &Frame,
        stream_type: StreamType,
    ) -> Result<(u32, u32, usize, Vec<u8>)> {
        use self::StreamType::*;

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
    }
}

impl Drop for Sensor {
    fn drop(&mut self) {
        self.stop_all();
    }
}
