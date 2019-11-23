use self::StreamType::*;
use super::Sensor;
use crate::frame::Frame;
use model::{Error, Result, StreamType};
use wrapper::*;

#[cfg(feature = "godot")]
use model::gdnative;

impl Sensor {
    fn get_img_bytes(
        &mut self,
        frame: &Frame,
        stream_type: StreamType,
    ) -> Result<(u32, u32, usize, Vec<u8>)> {
        let img_frame = frame.get_img_frame(stream_type)?;
        let index = get_img_frame_index(img_frame)?;
        let frame_index = self.indexes.get_mut(&stream_type).unwrap();
        if frame_index != &index {
            *frame_index = index;

            get_bytes(img_frame)
        } else {
            Err(Error::NoNewFrameError)
        }
    }

    pub fn has_new_frame(
        &mut self,
        img_frame: &ImageFrame,
        stream_type: StreamType,
    ) -> Result<bool> {
        let index = get_img_frame_index(*img_frame)?;
        let frame_index = self.indexes.get_mut(&stream_type).unwrap();
        if frame_index != &index {
            *frame_index = index;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[cfg(feature = "godot")]
    pub fn get_img(&mut self, frame: &Frame, stream_type: StreamType) -> Result<gdnative::Image> {
        let img_frame = frame.get_img_frame(stream_type)?;
        let index = get_img_frame_index(img_frame)?;
        let frame_index = self.indexes.get_mut(&stream_type).unwrap();
        if frame_index != &index {
            *frame_index = index;

            get_img(img_frame, stream_type)
        } else {
            Err(Error::NoNewFrameError)
        }
    }

    pub fn get_img_dimensions(&self, img_frame: &ImageFrame) -> Result<(u32, u32)> {
        get_img_frame_dimensions(*img_frame)
    }

    pub fn get_img_byte_length(&self, img_frame: &ImageFrame) -> Result<usize> {
        get_img_frame_byte_length(*img_frame)
    }

    pub fn copy_img_bytes(&mut self, img_frame: &ImageFrame, bytes_ptr: *mut u8) -> Result<()> {
        copy_bytes(*img_frame, bytes_ptr)
    }

    pub fn get_color_index(&self) -> i32 {
        self.indexes[&StreamType::Color]
    }
    pub fn get_depth_index(&self) -> i32 {
        self.indexes[&StreamType::Depth]
    }
    pub fn get_masked_color_index(&self) -> i32 {
        self.indexes[&StreamType::MaskedColor]
    }
    pub fn get_infrared_index(&self) -> i32 {
        self.indexes[&StreamType::Infrared]
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
    pub fn get_infrared_bytes(&mut self, frame: &Frame) -> Result<(u32, u32, usize, Vec<u8>)> {
        self.get_img_bytes(&frame, StreamType::Infrared)
    }
    pub fn start_color_stream(&mut self) -> Result<()> {
        self.start_stream(StreamType::Color)
    }
    pub fn start_infrared_stream(&mut self) -> Result<()> {
        self.start_stream(StreamType::Infrared)
    }
    pub fn start_masked_color_stream(&mut self) -> Result<()> {
        self.start_stream(StreamType::MaskedColor)
    }
    pub fn start_depth_stream(&mut self) -> Result<()> {
        self.start_stream(StreamType::Depth)
    }
}
