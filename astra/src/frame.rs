use super::*;
use crate::err::Error;

pub struct Frame {
    frame: AstraFrame,
}

impl Frame {
    pub fn new(reader: Reader) -> Result<Frame, Error> {
        if let Some(mut frame) = get_frame(reader) {
            Ok(Frame { frame: frame })
        } else {
            Err(Error::CouldNotGetFrameError)
        }
    }

    pub fn get_color_frame(&self) -> ColorFrame {
        get_color_frame(self.frame)
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        close_frame(&mut self.frame);
    }
}
