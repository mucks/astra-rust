use crate::frame::Frame;
use model::{Body, StreamType};
use model::{Error, Result};
use std::collections::HashMap;
use wrapper::*;

mod img;

#[derive(Default)]
pub struct Sensor {
    stream_set: Option<StreamSet>,
    reader: Option<Reader>,
    streams: HashMap<StreamType, Stream>,
    indexes: HashMap<StreamType, i32>,
}

impl Sensor {
    pub fn new() -> Self {
        Self::default()
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

    pub fn update(&self) -> Result<Frame> {
        update()?;
        if let Some(reader) = self.reader {
            Frame::new(reader)
        } else {
            Err(Error::SensorNotInitializedError)
        }
    }
    fn init_indexes(&mut self) {
        self.indexes = [
            (StreamType::Color, -1),
            (StreamType::Body, -1),
            (StreamType::MaskedColor, -1),
            (StreamType::Depth, -1),
            (StreamType::Infrared, -1),
        ]
        .iter()
        .cloned()
        .collect();
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
    pub fn get_body_index(&self) -> i32 {
        self.indexes[&StreamType::Body]
    }

    pub fn start_body_stream(&mut self) -> Result<()> {
        self.start_stream(StreamType::Body)
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
            terminate()?;
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
}

impl Drop for Sensor {
    fn drop(&mut self) {
        self.stop_all();
    }
}
