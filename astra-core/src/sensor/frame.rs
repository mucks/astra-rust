use crate::err::Error;
use model::FrameType;
use wrapper::*;

pub struct Frame {
    frame: AstraFrame,
}

impl Frame {
    pub fn new(reader: Reader) -> Result<Frame, Error> {
        if let Some(frame) = get_frame(reader) {
            Ok(Frame { frame })
        } else {
            Err(Error::CouldNotGetFrameError(FrameType::Frame))
        }
    }

    pub fn get_color_frame(&self) -> ColorFrame {
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
