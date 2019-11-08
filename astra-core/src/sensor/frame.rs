use model::{ColorMeta, FrameType};
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

    pub fn get_color_frame(&self) -> Result<ColorFrame> {
        get_color_frame(self.frame)
    }
    pub fn get_body_frame(&self) -> BodyFrame {
        get_body_frame(self.frame)
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        close_frame(&mut self.frame);
    }
}
