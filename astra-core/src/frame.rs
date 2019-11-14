use model::{ColorMeta, FrameType, StreamType};
use model::{Error, Result};
use wrapper::*;

pub struct Frame {
    frame: AstraFrame,
    pub color_meta: Option<ColorMeta>,
}

impl Frame {
    pub fn new(reader: Reader) -> Result<Frame> {
        if let Some(frame) = get_frame(reader) {
            Ok(Frame {
                frame: frame,
                color_meta: None,
            })
        } else {
            Err(Error::CouldNotGetFrameError(FrameType::Frame))
        }
    }

    pub fn get_img_frame(&self, stream_type: StreamType) -> Result<ImageFrame> {
        get_img_frame(stream_type, self.frame)
    }
    pub fn get_infrared_frame(&self) -> Result<InfraredFrame> {
        get_img_frame(StreamType::Infrared, self.frame)
    }
    pub fn get_color_frame(&self) -> Result<ColorFrame> {
        get_img_frame(StreamType::Color, self.frame)
    }
    pub fn get_body_frame(&self) -> Result<BodyFrame> {
        get_body_frame(self.frame)
    }
    pub fn get_masked_color_frame(&self) -> Result<MaskedColorFrame> {
        get_img_frame(StreamType::MaskedColor, self.frame)
    }
    pub fn get_depth_frame(&self) -> Result<DepthFrame> {
        get_img_frame(StreamType::Depth, self.frame)
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        close_frame(&mut self.frame);
    }
}
